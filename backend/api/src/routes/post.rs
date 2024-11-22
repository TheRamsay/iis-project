use anyhow::Ok;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::post::{
    create_post::{CreatePostInput, CreatePostUseCase},
    delete_post::{DeletePostInput, DeletePostUseCase},
    get_post::{GetPostInput, GetPostUseCase},
    upload_image::{UploadImageInput, UploadImageUseCase},
};
use uuid::Uuid;
use validator::ValidationErrors;

use crate::{
    extractors::{auth_extractor::AuthUser, json_extractor::Json},
    AppState,
};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
};
use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        user::UserType,
    },
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
    created_at: DateTime<Utc>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeletePostResponse {
    success: bool,
}

async fn delete_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    user: AuthUser,
) -> AppResult<Json<DeletePostResponse>> {
    let post_use_case = GetPostUseCase::new(state.post_repository.clone());
    let delete_use_case = DeletePostUseCase::new(state.post_repository.clone());
    let post = post_use_case.execute(GetPostInput { id }).await?;

    if post.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    if user.id.into() != post.unwrap().post.author_id
        && user.role != UserType::Administrator
        && user.role != UserType::Moderator
    {
        return Err(AppError::Unauthorized("Unauthorized".into()));
    }

    let result = delete_use_case.execute(DeletePostInput { id }).await?;

    if result.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    anyhow::Result::Ok(Json(DeletePostResponse { success: true }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UploadImageRequest {
    image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UploadImageResponse {
    link: String,
}

async fn upload_image(
    state: State<AppState>,
    Json(payload): Json<UploadImageRequest>,
) -> AppResult<Json<UploadImageResponse>> {
    let upload_image_use_case = UploadImageUseCase::new(state.cloudinary_repository.clone());

    let input = UploadImageInput {
        image: payload.image,
    };

    let output = upload_image_use_case.execute(input).await?;

    anyhow::Result::Ok(Json(UploadImageResponse { link: output.url }))
}

pub fn post_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post))
        .route("/:id", delete(delete_post))
        .route("/upload_image", post(upload_image))
}
