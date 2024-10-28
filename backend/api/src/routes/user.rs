use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use models::{
    domain::user::UserType,
    errors::{AppError, AppResult},
    schema::user,
};
use serde::{Deserialize, Serialize};
use usecase::user::{
    create_user::{CreateUserInput, CreateUserUseCase},
    get_user::{GetUserInput, GetUserUseCase},
};
use uuid::Uuid;

use crate::{extractors::json_extractor::Json, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateUserRequest {
    display_name: String,
    username: String,
    email: String,
    avatar_url: Option<String>,
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
        CreateUserUseCase::new(state.user_repository.clone(), state.wall_repository.clone());

    let input = CreateUserInput {
        display_name: payload.display_name,
        username: payload.username,
        email: payload.email,
        avatar_url: payload.avatar_url,
        user_type: models::domain::user::UserType::Regular,
    };

    let output = user_usercase.execute(input).await?;

    anyhow::Result::Ok(Json(CreateUserResponse { id: output.id }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn user_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user))
}
