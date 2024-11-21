use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    routing::{delete, get, patch, post, put, Route},
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use models::{
    domain::user::{User, UserType},
    errors::{AppError, AppResult},
};
use repository::user_repository::UserRepository;
use serde::{Deserialize, Serialize};
use usecase::user::{
    block_user::{BlockUserInput, BlockUserUseCase},
    get_user::{GetUserInput, GetUserUseCase},
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
    display_name: String,
    username: String,
    email: String,
    avatar_url: Option<String>,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateUserResponse {
    id: Uuid,
}

async fn create_user(
    state: State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<Json<CreateUserResponse>> {
    let user_usercase =
        RegisterUserUseCase::new(state.user_repository.clone(), state.wall_repository.clone());

    let input = RegisterUserInput {
        display_name: payload.display_name,
        username: payload.username,
        email: payload.email,
        avatar_url: payload.avatar_url,
        user_type: models::domain::user::UserType::Regular,
        password: payload.password,
    };

    let output = user_usercase.execute(input).await?;

    anyhow::Result::Ok(Json(CreateUserResponse { id: output.id }))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct GetUserResponse {
    id: Uuid,
    display_name: String,
    username: String,
    email: String,
    avatar_url: Option<String>,
    user_type: String,
}

async fn get_user(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<GetUserResponse>> {
    let user_usercase = GetUserUseCase::new(state.user_repository.clone());

    let user = user_usercase.execute(GetUserInput { id }).await?;

    if let Some(user) = user {
        anyhow::Result::Ok(Json(GetUserResponse {
            id: user.id.id,
            display_name: user.display_name,
            username: user.username,
            email: user.email.value,
            avatar_url: user.avatar_url,
            user_type: match user.user_type {
                models::domain::user::UserType::Regular => "Regular".to_string(),
                models::domain::user::UserType::Administrator => "Admin".to_string(),
                models::domain::user::UserType::Moderator => "Moderator".to_string(),
            },
        }))
    } else {
        Err(AppError::NotFound("User".into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct MeResponse {
    id: Uuid,
    username: String,
    user_type: UserType,
}

async fn me(user: AuthUser) -> AppResult<Json<MeResponse>> {
    Ok(Json(MeResponse {
        username: user.username,
        id: user.id,
        user_type: user.role,
    }))
}

async fn block_user(
    state: State<AppState>,
    admin: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
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

    if (admin.role == models::domain::user::UserType::Regular)
        || (admin.id == user.id.into())
        || (admin.role as i32 <= user.user_type as i32)
    {
        return Err(AppError::Unauthorized("You can't block this user".into()));
    }

    block_user_usecase
        .execute(BlockUserInput { user_id: id })
        .await?;

    Ok(())
}

async fn delete_user(
    state: State<AppState>,
    admin: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    if admin.role != models::domain::user::UserType::Administrator {
        return Err(AppError::Unauthorized("You can't delete this user".into()));
    }

    let get_user_usercase = GetUserUseCase::new(state.user_repository.clone());

    let user = get_user_usercase.execute(GetUserInput { id }).await?;

    if user.is_none() {
        return Err(AppError::NotFound("User".into()));
    }

    let user = user.unwrap();

    if admin.id == user.id.clone().into() {
        return Err(AppError::Unauthorized("You can't delete yourself".into()));
    }

    state.user_repository.delete(user.id).await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateUserRequest {
    display_name: String,
    username: String,
    email: String,
    avatar_url: Option<String>,
    password: String,
    user_type: UserType,
}

async fn update_user(
    state: State<AppState>,
    mut jar: CookieJar,
    actor: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<(CookieJar, ())> {
    let is_admin = actor.role == UserType::Administrator;
    let modifies_self = actor.id == id;

    // Check if the actor is an admin or the user being modified
    if (!is_admin) && (!modifies_self) {
        return Err(AppError::Unauthorized("You can't modify this user".into()));
    }

    let get_user_usecase = GetUserUseCase::new(state.user_repository.clone());
    let user = get_user_usecase
        .execute(GetUserInput { id })
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // If actor tries to change user type, he must be an admin
    if !is_admin && user.user_type != payload.user_type {
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
        let new_jwt_str = AuthUser::new(
            updated.id.into(),
            updated.username.clone(),
            updated.user_type,
        )
        .to_jwt();

        println!("{:?}", jar);
        let old_jwt_str = jar.get("jwt").map(|cookie| cookie.value().to_string());
        println!("old_jwt_str: {:?}", old_jwt_str);

        // Blacklist old token
        if let Some(old_jwt_str) = old_jwt_str {
            let old_jwt = AuthUser::from_jwt(&old_jwt_str)?;

            blacklist_token(&state.redis_client, &old_jwt_str, old_jwt.exp)
                .map_err(|e| AppError::Anyhow(anyhow!(e)))?;
        }

        let cookie = Cookie::build(("jwt", new_jwt_str))
            .same_site(axum_extra::extract::cookie::SameSite::Strict)
            .http_only(true)
            .secure(true);

        jar = jar.add(cookie);
    }

    std::result::Result::Ok((jar, ()))
}
pub fn user_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .route("/me", get(me))
        .route("/:id/block", get(block_user))
        .route("/:id", delete(delete_user))
        .route("/:id", put(update_user))
}
