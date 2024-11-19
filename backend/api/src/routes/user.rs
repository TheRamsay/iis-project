use axum::{
    extract::{Path, State},
    routing::{get, post},
};
use models::errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use usecase::user::{
    block_user::{BlockUserInput, BlockUserUseCase},
    get_user::{GetUserInput, GetUserUseCase},
    register_user::{RegisterUserInput, RegisterUserUseCase},
};
use uuid::Uuid;

use crate::{
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
    username: String,
    id: Uuid,
}

async fn me(user: AuthUser) -> AppResult<Json<MeResponse>> {
    Ok(Json(MeResponse {
        username: user.username,
        id: user.id,
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

pub fn user_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user))
        .route("/me", get(me))
        .route("/:id/block", get(block_user))
}
