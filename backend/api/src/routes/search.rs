use argon2::Params;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post},
};
use models::{
    domain::{group::Group, user::User},
    errors::AppResult,
};
use serde::{Deserialize, Serialize};

use usecase::{
    group::search_group::SearchGroupUseCase,
    post_tag::{
        create_post_tag::{CreatePostTagInput, CreatePostTagUseCase},
        delete_tag::{DeletePostTagInput, DeletePostTagUseCase},
        search_tag::{self, SearchPostTagInput, SearchPostTagUseCase},
    },
    user::search_user_by_username::SearchUserByUsernameUseCase,
};
use uuid::Uuid;

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

use super::{
    group::{Admin, GetGroupResponse},
    post_tag::SearchPostTag,
    user::GetUserResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SearchRequest {
    query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchResponse {
    users: Vec<GetUserResponse>,
    groups: Vec<GetGroupResponse>,
    tags: Vec<SearchPostTag>,
}

async fn search(
    state: State<AppState>,
    Query(params): Query<SearchRequest>,
) -> AppResult<Json<SearchResponse>> {
    let search_tag_use_case = SearchPostTagUseCase::new(state.post_tag_repository.clone());
    let search_group_use_case = SearchGroupUseCase::new(state.group_repository.clone());
    let search_user_use_case = SearchUserByUsernameUseCase::new(state.user_repository.clone());

    let input = search_tag::SearchPostTagInput {
        query: params.query.clone(),
    };

    let tags: Vec<_> = search_tag_use_case.execute(input).await?.tags;

    let input = usecase::group::search_group::SearchGroupInput {
        query: params.query.clone(),
        filter_where_member: None,
    };

    let groups = search_group_use_case.execute(input).await?;

    let input = usecase::user::search_user_by_username::SearchUserByUsernameInput {
        username: params.query.clone(),
    };

    let users = search_user_use_case
        .execute(input)
        .await?
        .unwrap_or_default();

    Ok(Json(SearchResponse {
        users: users
            .into_iter()
            .map(|user| GetUserResponse {
                id: user.id.into(),
                display_name: user.display_name,
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
                user_type: user.user_type.to_string(),
                wall_id: user.wall_id.into(),
                is_blocked: user.is_blocked,
            })
            .collect(),
        groups: groups
            .groups // Assuming `SearchGroupOutput` has a field `groups` that is iterable
            .into_iter()
            .map(|group| GetGroupResponse {
                id: group.0.id.into(),
                name: group.0.name,
                admin: Admin {
                    id: group.1.id.into(),
                    display_name: group.1.display_name,
                    username: group.1.username,
                    email: group.1.email,
                    avatar_url: group.1.avatar_url,
                    user_type: group.1.user_type,
                },
                wall_id: group.0.wall_id.into(),
            })
            .collect(),
        tags: tags
            .into_iter()
            .map(|tag| SearchPostTag {
                post_id: tag.post_id.into(),
                tag: tag.tag,
            })
            .collect(),
    }))
}

pub fn search_routes() -> axum::Router<crate::AppState> {
    axum::Router::new().route("/", get(search))
}
