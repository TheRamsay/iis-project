use axum::{
    extract::{Path, State},
    routing::{get, post},
};
use models::{
    domain::{
        group_join_request::{self, GroupJoinRequestStatus},
        group_member::GroupMember,
    },
    errors::{AppError, AppResult},
    tests::post,
};
use repository::{
    group_join_request_repository::GroupJoinRequestRepository,
    group_member_repository::GroupMemberRepository,
};
use serde::{Deserialize, Serialize};
use usecase::group::resolve_group_join_request::{self, ResolveGroupJoinRequestInput};
use uuid::Uuid;

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

async fn accept(
    state: State<AppState>,
    user: AuthUser,
    Path(group_join_request_id): Path<Uuid>,
) -> AppResult<()> {
    let resolve_group_join_request_usecase =
        resolve_group_join_request::ResolveGroupJoinRequestUseCase::new(
            state.group_repository.clone(),
            state.group_join_request_repository.clone(),
            state.group_member_repository.clone(),
        );

    let input = ResolveGroupJoinRequestInput {
        id: group_join_request_id,
        user_id: user.id,
        new_status: GroupJoinRequestStatus::Accepted,
    };

    resolve_group_join_request_usecase.execute(input).await?;

    Ok(())
}

async fn reject(
    state: State<AppState>,
    user: AuthUser,
    Path(group_join_request_id): Path<Uuid>,
) -> AppResult<()> {
    let resolve_group_join_request_usecase =
        resolve_group_join_request::ResolveGroupJoinRequestUseCase::new(
            state.group_repository.clone(),
            state.group_join_request_repository.clone(),
            state.group_member_repository.clone(),
        );

    let input = ResolveGroupJoinRequestInput {
        id: group_join_request_id,
        user_id: user.id,
        new_status: GroupJoinRequestStatus::Rejected,
    };

    resolve_group_join_request_usecase.execute(input).await?;

    Ok(())
}

pub fn group_join_request_router() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/:id/approve", get(accept))
        .route("/:id/reject", get(reject))
}
