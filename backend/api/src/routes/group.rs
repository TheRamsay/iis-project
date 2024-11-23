use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
};
use models::{
    domain::{
        group_join_request,
        user::{User, UserType},
    },
    errors::{AppError, AppResult},
    schema::user,
};
use repository::{group_repository::GroupRepository, user_repository::UserRepository};
use serde::{Deserialize, Serialize};
use usecase::{
    group::{
        add_user_to_group,
        create_group::{CreateGroupInput, CreateGroupUseCase},
        get_group::{GetGroupInput, GetGroupUseCase},
        join_group::{JoinGroupInput, JoinGroupUseCase},
        leave_group::{LeaveGroupInput, LeaveGroupUseCase},
        remove_user_from_group,
        search_group::{SearchGroupInput, SearchGroupOutput, SearchGroupUseCase},
    },
    user::{
        get_user::{GetUserInput, GetUserUseCase},
        register_user::{RegisterUserInput, RegisterUserUseCase},
    },
};
use uuid::Uuid;

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CreateGroupRequest {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateGroupResponse {
    id: Uuid,
}

async fn create_group(
    state: State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateGroupRequest>,
) -> AppResult<Json<CreateGroupResponse>> {
    let group_usecace = CreateGroupUseCase::new(
        state.group_repository.clone(),
        state.wall_repository.clone(),
        state.group_member_repository.clone(),
    );

    let input = CreateGroupInput {
        name: payload.name,
        admin_id: user.id,
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
                email: group.admin.email,
                avatar_url: group.admin.avatar_url,
                user_type: group.admin.user_type,
            },
            wall_id: group.group.wall_id.into(),
        }))
    } else {
        Err(AppError::NotFound("Group".into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SearchGroupRequest {
    query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchGroupResponse {
    groups: Vec<GetGroupResponse>,
}

async fn search_group(
    state: State<AppState>,
    Query(params): Query<SearchGroupRequest>,
) -> AppResult<Json<SearchGroupResponse>> {
    let group_usecace = SearchGroupUseCase::new(state.group_repository.clone());

    let input = SearchGroupInput {
        query: params.query.unwrap_or(String::new()),
    };

    let output = group_usecace.execute(input).await?;

    anyhow::Result::Ok(Json(SearchGroupResponse {
        groups: output
            .groups
            .into_iter()
            .map(|(group, admin)| GetGroupResponse {
                id: group.id.into(),
                name: group.name,
                admin: Admin {
                    id: admin.id.into(),
                    display_name: admin.display_name,
                    username: admin.username,
                    email: admin.email,
                    avatar_url: admin.avatar_url,
                    user_type: admin.user_type,
                },
                wall_id: group.wall_id.into(),
            })
            .collect(),
    }))
}

async fn join_group(
    state: State<AppState>,
    user: AuthUser,
    Path(group_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let group_member_repository = state.group_member_repository.clone();
    let group_repository = state.group_repository.clone();
    let group_join_request_repository = state.group_join_request_repository.clone();

    let use_case = JoinGroupUseCase::new(
        group_repository,
        group_join_request_repository,
        group_member_repository,
    );

    let input = JoinGroupInput {
        user_id: user.id,
        group_id,
    };

    use_case.execute(input).await?;

    Ok(())
}

async fn leave_group(
    state: State<AppState>,
    user: AuthUser,
    Path(group_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let group_member_repository = state.group_member_repository.clone();
    let group_repository = state.group_repository.clone();

    let use_case = LeaveGroupUseCase::new(group_repository, group_member_repository);

    let input = LeaveGroupInput {
        user_id: user.id,
        group_id,
    };

    use_case.execute(input).await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AddUserToGroupRequest {
    user_id: Uuid,
}

async fn add_user(
    state: State<AppState>,
    user: AuthUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<AddUserToGroupRequest>,
) -> AppResult<()> {
    let add_user_to_group_usecase = add_user_to_group::AddUserToGroupUseCase::new(
        state.group_repository.clone(),
        state.group_member_repository.clone(),
    );

    let input = add_user_to_group::AddUserToGroupInput {
        user_id: payload.user_id,
        group_id,
        admin_id: user.id,
    };

    add_user_to_group_usecase.execute(input).await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RemoveUserFromGroupRequest {
    user_id: Uuid,
}

async fn remove_user(
    state: State<AppState>,
    user: AuthUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<RemoveUserFromGroupRequest>,
) -> AppResult<()> {
    let remove_user_to_group_usecase = remove_user_from_group::RemoveUserToGroupUseCase::new(
        state.group_repository.clone(),
        state.group_member_repository.clone(),
    );

    let input = remove_user_from_group::RemoveUserToGroupInput {
        user_id: payload.user_id,
        group_id,
        admin_id: user.id,
    };

    remove_user_to_group_usecase.execute(input).await?;

    Ok(())
}

pub fn group_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", get(search_group))
        .route("/", post(create_group))
        .route("/:id", get(get_group))
        .route("/:id/join", get(join_group))
        .route("/:id/leave", get(leave_group))
        .route("/:id/remove_user", post(remove_user))
        .route("/:id/add_user", post(add_user))
}
