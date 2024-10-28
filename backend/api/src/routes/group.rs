use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use models::{
    domain::user::{User, UserType},
    errors::{AppError, AppResult},
    schema::user,
};
use serde::{Deserialize, Serialize};
use usecase::{
    group::{
        create_group::{CreateGroupInput, CreateGroupUseCase},
        get_group::{GetGroupInput, GetGroupUseCase},
    },
    user::{
        create_user::{CreateUserInput, CreateUserUseCase},
        get_user::{GetUserInput, GetUserUseCase},
    },
};
use uuid::Uuid;

use crate::{extractors::json_extractor::Json, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateGroupRequest {
    name: String,
    admin_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateGroupResponse {
    id: Uuid,
}

async fn create_group(
    state: State<AppState>,
    Json(payload): Json<CreateGroupRequest>,
) -> AppResult<Json<CreateGroupResponse>> {
    let group_usecace = CreateGroupUseCase::new(
        state.group_repository.clone(),
        state.wall_repository.clone(),
    );

    let input = CreateGroupInput {
        name: payload.name,
        admin_id: payload.admin_id,
    };

    let output = group_usecace.execute(input).await?;

    anyhow::Result::Ok(Json(CreateGroupResponse { id: output.id }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Admin {
    id: Uuid,
    display_name: String,
    username: String,
    email: String,
    avatar_url: Option<String>,
    user_type: UserType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetGroupResponse {
    id: Uuid,
    name: String,
    admin: Admin,
    wall_id: Uuid,
}

async fn get_group(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<GetGroupResponse>> {
    let group_usecace = GetGroupUseCase::new(state.group_repository.clone());

    let group = group_usecace.execute(GetGroupInput { id }).await?;

    if let Some(group) = group {
        anyhow::Result::Ok(Json(GetGroupResponse {
            id: group.group.id.into(),
            name: group.group.name,
            admin: Admin {
                id: group.admin.id.into(),
                display_name: group.admin.display_name,
                username: group.admin.username,
                email: group.admin.email.value,
                avatar_url: group.admin.avatar_url,
                user_type: group.admin.user_type,
            },
            wall_id: group.group.wall_id.into(),
        }))
    } else {
        Err(AppError::NotFound("Group".into()))
    }
}

pub fn group_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_group))
        .route("/:id", get(get_group))
}
