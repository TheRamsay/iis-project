use serde::{Deserialize, Serialize};
use usecase::post::{
    create_post::{CreatePostInput, CreatePostUseCase},
    get_post::{GetPostInput, GetPostUseCase},
};
use uuid::Uuid;
use validator::ValidationErrors;

use crate::{extractors::json_extractor::Json, AppState};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use models::{
    domain::post::{Post, PostType, PostVisibilityType},
    errors::{AppError, AppResult},
    schema::post,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreatePostRequest {
    title: String,
    description: String,
    author_id: Uuid,
    post_type: String,
    content_url: String,
    visibility: String,
    location_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreatePostResponse {
    id: Uuid,
}

async fn create_post(
    state: State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> AppResult<Json<CreatePostResponse>> {
    let post_usecase = CreatePostUseCase::new(state.post_repository.clone());

    let input = CreatePostInput {
        title: payload.title,
        description: payload.description,
        author_id: payload.author_id,
        post_type: match payload.post_type.as_str() {
            "Photo" => PostType::Photo,
            _ => return Err(AppError::ValidationError(ValidationErrors::new())),
        },
        content_url: payload.content_url,
        visibility: match payload.visibility.as_str() {
            "Public" => PostVisibilityType::Public,
            "Private" => PostVisibilityType::Private,
            _ => return Err(AppError::ValidationError(ValidationErrors::new())),
        },
        location_id: payload.location_id,
    };

    let output = post_usecase.execute(input).await?;

    anyhow::Result::Ok(Json(CreatePostResponse { id: output.id }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetPostResponse {
    id: Uuid,
    title: String,
    description: String,
    post_type: String,
    author_id: Uuid,
    content_url: String,
    visibility: String,
    location_id: Option<Uuid>,
    created_at: chrono::NaiveDateTime,
}

async fn get_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<GetPostResponse>> {
    let post_usecase = GetPostUseCase::new(state.post_repository.clone());

    let post = post_usecase.execute(GetPostInput { id }).await?;

    if let Some(post) = post {
        anyhow::Result::Ok(Json(GetPostResponse {
            id: post.post.id.into(),
            title: post.post.title,
            description: post.post.description,
            post_type: match post.post.post_type {
                PostType::Photo => "Photo".into(),
            },
            author_id: post.post.author_id.into(),
            content_url: post.post.content_url,
            visibility: match post.post.visibility {
                PostVisibilityType::Public => "Public".into(),
                PostVisibilityType::Private => "Private".into(),
            },
            location_id: post.post.location_id.map(|id| id.into()),
            created_at: post.post.created_at,
        }))
    } else {
        Err(AppError::NotFound("Post".into()))
    }
}

pub fn post_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post))
}
