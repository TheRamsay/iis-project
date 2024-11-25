use anyhow::anyhow;
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
};
use axum_extra::extract::{
    cookie::{Cookie, Expiration},
    CookieJar,
};
use models::{
    domain::user::UserType,
    errors::{AppError, AppResult},
};
use repository::user_repository::UserRepository;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use usecase::user::{
    block_user::{BlockUserInput, BlockUserUseCase},
    get_all_users::{GetAllUsersInput, GetAllUsersUseCase},
    get_user::{GetUserInput, GetUserUseCase},
    get_user_by_username::{GetUserByUsernameInput, GetUserByUsernameUseCase},
    register_user::{RegisterUserInput, RegisterUserUseCase},
    update_user::{UpdateUserInput, UpdateUserUseCase},
};
use uuid::Uuid;

use crate::{
    auth::jwt::blacklist_token,
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CreateUserRequest {
    username: String,
    description: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
    password: String,
    user_type: UserType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateUserResponse {
    id: Uuid,
}

async fn create_user(
    state: State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<Json<CreateUserResponse>> {
    if user.role != UserType::Administrator {
        return Err(AppError::Unauthorized("You can't create a user".into()));
    }

    let user_usercase =
        RegisterUserUseCase::new(state.user_repository.clone(), state.wall_repository.clone());

    let input = RegisterUserInput {
        username: payload.username,
        description: payload.description,
        email: payload.email,
        avatar_url: payload.avatar_url,
        user_type: payload.user_type,
        password: payload.password,
    };

    let output = user_usercase.execute(input).await?;

    anyhow::Result::Ok(Json(CreateUserResponse { id: output.id }))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub username: String,
    pub description: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub user_type: String,
    pub wall_id: Uuid,
    pub is_blocked: bool,
}

async fn get_user_by_id(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<GetUserResponse>> {
    let user_usercase = GetUserUseCase::new(state.user_repository.clone());

    let user = user_usercase
        .execute(GetUserInput { id })
        .await?;

    if let Some(user) = user {
        anyhow::Result::Ok(Json(GetUserResponse {
            id: user.id.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            description: user.description,
            user_type: user.user_type.to_string(),
            wall_id: user.wall_id.id,
            is_blocked: user.is_blocked,
        }))
    } else {
        Err(AppError::NotFound("User".into()))
    }
}


async fn get_user_by_username(
    state: State<AppState>,
    Path(username): Path<String>,
) -> AppResult<Json<GetUserResponse>> {
    let user_usercase = GetUserByUsernameUseCase::new(state.user_repository.clone());

    let user = user_usercase
        .execute(GetUserByUsernameInput { username })
        .await?;

    if let Some(user) = user {
        anyhow::Result::Ok(Json(GetUserResponse {
            id: user.id.id,
            username: user.username,
            description: user.description,
            email: user.email,
            avatar_url: user.avatar_url,
            user_type: user.user_type.to_string(),
            wall_id: user.wall_id.id,
            is_blocked: user.is_blocked,
        }))
    } else {
        Err(AppError::NotFound("User".into()))
    }
}

async fn me(state: State<AppState>, user: AuthUser) -> AppResult<Json<GetUserResponse>> {
    let user_usercase = GetUserUseCase::new(state.user_repository.clone());

    let user = user_usercase.execute(GetUserInput { id: user.id }).await?;

    if let Some(user) = user {
        anyhow::Result::Ok(Json(GetUserResponse {
            id: user.id.id,
            description: user.description,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            user_type: user.user_type.to_string(),
            wall_id: user.wall_id.id,
            is_blocked: user.is_blocked,
        }))
    } else {
        Err(AppError::NotFound("User".into()))
    }
}

async fn block_user(
    state: State<AppState>,
    mut jar: CookieJar,
    actor: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<(CookieJar, ())> {
    let get_user_usercase = GetUserUseCase::new(state.user_repository.clone());
    let block_user_usecase = BlockUserUseCase::new(state.user_repository.clone());

    let user = get_user_usercase.execute(GetUserInput { id }).await?;

    if user.is_none() {
        return Err(AppError::NotFound("User".into()));
    }

    let user = user.unwrap();

    if user.is_blocked {
        return Err(AppError::BadRequest("User is already blocked".into()));
    }

    let self_block = actor.id == user.id.into();
    if actor.role.is_regular()
        || actor.role.has_lower_or_same_privilege_as(&user.user_type)
        || self_block
    {
        return Err(AppError::Unauthorized(
            "You do not have sufficient privileges to block this user.".into(),
        ));
    }

    block_user_usecase
        .execute(BlockUserInput { user_id: id })
        .await?;

    let old_jwt_str = jar
        .get("jwt")
        .map(|cookie| cookie.value().to_string())
        .expect("JWT cookie not found");

    blacklist_token(&state.redis_client, &old_jwt_str, actor.exp)
        .map_err(|e| AppError::Anyhow(anyhow!(e)))?;

    jar = jar.remove(Cookie::from("jwt"));

    Ok((jar, ()))
}

async fn delete_user(
    state: State<AppState>,
    mut jar: CookieJar,
    actor: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<(CookieJar, ())> {
    if actor.role != models::domain::user::UserType::Administrator {
        return Err(AppError::Unauthorized("You can't delete this user".into()));
    }

    let get_user_usercase = GetUserUseCase::new(state.user_repository.clone());

    let user = get_user_usercase.execute(GetUserInput { id }).await?;

    if user.is_none() {
        return Err(AppError::NotFound("User".into()));
    }

    let user = user.unwrap();

    if actor.id == user.id.clone().into() {
        return Err(AppError::Unauthorized("You can't delete yourself".into()));
    }

    state.user_repository.delete(user.id).await?;

    let old_jwt_str = jar
        .get("jwt")
        .map(|cookie| cookie.value().to_string())
        .expect("JWT cookie not found");

    blacklist_token(&state.redis_client, &old_jwt_str, actor.exp)
        .map_err(|e| AppError::Anyhow(anyhow!(e)))?;

    jar = jar.remove(Cookie::from("jwt"));

    Ok((jar, ()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateUserRequest {
    display_name: Option<String>,
    username: String,
    email: Option<String>,
    avatar_url: Option<String>,
    password: Option<String>,
    user_type: UserType,
}

async fn update_user(
    state: State<AppState>,
    mut jar: CookieJar,
    actor: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<(CookieJar, ())> {
    let modifies_self = actor.id == id;

    // Check if the actor is an admin or the user being modified
    if actor.role.is_administrator() && !modifies_self {
        return Err(AppError::Unauthorized("You can't modify this user".into()));
    }

    let get_user_usecase = GetUserUseCase::new(state.user_repository.clone());
    let user = get_user_usecase
        .execute(GetUserInput { id })
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // If actor tries to change user type, he must be an admin
    if !actor.role.is_administrator() && !user.user_type.has_same_privilege_as(&payload.user_type) {
        return Err(AppError::Unauthorized("You can't change user type".into()));
    }

    let update_user_usecase = UpdateUserUseCase::new(state.user_repository.clone());
    let updated = update_user_usecase
        .execute(UpdateUserInput {
            id,
            email: payload.email,
            username: payload.username,
            display_name: payload.display_name,
            avatar_url: payload.avatar_url,
            user_type: payload.user_type,
            password: payload.password,
            user: user.clone(),
        })
        .await?;

    // If the user is modifying himself, update the jwt, otherwise, do nothing
    if modifies_self {
        let old_jwt_str = jar
            .get("jwt")
            .map(|cookie| cookie.value().to_string())
            .expect("JWT cookie not found");

        blacklist_token(&state.redis_client, &old_jwt_str, actor.exp)
            .map_err(|e| AppError::Anyhow(anyhow!(e)))?;

        let new_jwt = AuthUser::new(
            updated.id.into(),
            updated.username.clone(),
            updated.user_type,
        );

        let new_jwt_str = new_jwt.to_jwt(&state.jwt_secret);

        let cookie = Cookie::build(("jwt", new_jwt_str))
            .same_site(axum_extra::extract::cookie::SameSite::None)
            .path("/")
            .http_only(true)
            .expires(Expiration::DateTime(
                OffsetDateTime::from_unix_timestamp(new_jwt.exp as i64)
                    .map_err(|_| anyhow!("Failed to create expiration time"))?,
            ))
            .secure(true);

        jar = jar.add(cookie);
    }

    std::result::Result::Ok((jar, ()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllUsersRequest {
    role: Option<UserType>,
    is_blocked: Option<bool>,
    username: Option<String>,
}

async fn get_all_users(
    state: State<AppState>,
    Query(filters): Query<GetAllUsersRequest>,
) -> AppResult<Json<Vec<GetUserResponse>>> {
    let user_usercase = GetAllUsersUseCase::new(state.user_repository.clone());

    let users = user_usercase
        .execute(GetAllUsersInput {
            filter_role: filters.role,
            filter_is_blocked: filters.is_blocked,
            filter_username: filters.username,
        })
        .await?;

    let users = users
        .into_iter()
        .map(|user| GetUserResponse {
            id: user.id.id,
            description: user.description,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            user_type: user.user_type.to_string(),
            is_blocked: user.is_blocked,
            wall_id: user.wall_id.id,
        })
        .collect();

    anyhow::Result::Ok(Json(users))
}

pub fn user_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", get(get_all_users))
        .route("/", post(create_user))
        .route("/me", get(me))
        .route("/:username", get(get_user_by_username))
        .route("/id/:id", get(get_user_by_id))
        .route("/id/:id", delete(delete_user))
        .route("/id/:id", put(update_user))
        .route("/id/:id/block", get(block_user))
}
