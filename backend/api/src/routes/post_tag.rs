use argon2::Params;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post},
};
use models::errors::AppResult;
use serde::{Deserialize, Serialize};

use usecase::post_tag::{
    create_post_tag::{CreatePostTagInput, CreatePostTagUseCase},
    delete_tag::{DeletePostTagInput, DeletePostTagUseCase},
    search_tag::{SearchPostTagInput, SearchPostTagUseCase},
};
use uuid::Uuid;

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreatePostTagResponse {
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CreatePostTagRequest {
    tag: String,
    post_id: Uuid,
}

async fn create_tag(
    state: State<AppState>,
    Json(payload): Json<CreatePostTagRequest>,
) -> AppResult<Json<CreatePostTagResponse>> {
    let create_tag_use_case = CreatePostTagUseCase::new(state.post_tag_repository.clone());

    let input = CreatePostTagInput {
        tag: payload.tag,
        post_id: payload.post_id,
    };

    let result = create_tag_use_case.execute(input).await?;

    Ok(Json(CreatePostTagResponse { id: result.tag }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeletePostTagResponse {
    success: bool,
}

async fn delete_tag(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<DeletePostTagResponse>> {
    let delete_location_use_case = DeletePostTagUseCase::new(state.post_tag_repository.clone());

    let input = DeletePostTagInput { id };

    let result = delete_location_use_case.execute(input).await?;

    Ok(Json(DeletePostTagResponse {
        success: result.success,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SearchPostTagRequest {
    query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchPostTagResponse {
    tags: Vec<SearchPostTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchPostTag {
    post_id: Uuid,
    tag: String,
}

async fn search_tag(
    state: State<AppState>,
    // Json(payload): Json<SearchPostTagRequest>,
    Query(params): Query<SearchPostTagRequest>,
) -> AppResult<Json<SearchPostTagResponse>> {
    let search_location_use_case = SearchPostTagUseCase::new(state.post_tag_repository.clone());

    let input = SearchPostTagInput {
        query: params.query,
    };

    let result = search_location_use_case.execute(input).await?;

    let tags = result
        .tags
        .into_iter()
        .map(|tag| SearchPostTag {
            post_id: tag.post_id.into(),
            tag: tag.tag,
        })
        .collect();

    Ok(Json(SearchPostTagResponse { tags }))
}

pub fn post_tag_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_tag))
        .route("/:id", delete(delete_tag))
        .route("/search", get(search_tag))
}
