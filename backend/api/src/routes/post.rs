use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::post::{
    comment_post::{CommentPostInput, CommentPostUseCase},
    create_post::{CreatePostInput, CreatePostUseCase},
    delete_post::{DeletePostInput, DeletePostUseCase},
    get_comment::{GetCommentInput, GetCommentUseCase},
    get_post::{GetPostInput, GetPostUseCase},
    get_post_comments::{GetPostCommentsInput, GetPostCommentsUseCase},
    get_post_is_liked_by_user::{PostLikedByUserInput, PostLikedByUserUseCase},
    get_post_likes::{self, GetPostLikesInput},
    like_post::{LikePostInput, LikePostUseCase},
    uncomment_post::{UncommentPostInput, UncommentPostUseCase},
    unlike_post::{UnlikePostInput, UnlikePostUseCase},
    update_post::{UpdatePostInput, UpdatePostUseCase},
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
    routing::{delete, get, post, put},
};
use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        user::UserType,
    },
    errors::{AppError, AppResult},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreatePostRequest {
    title: String,
    description: String,
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
    user: AuthUser,
    Json(payload): Json<CreatePostRequest>,
) -> AppResult<Json<CreatePostResponse>> {
    let post_usecase = CreatePostUseCase::new(
        state.post_repository.clone(),
        state.wall_post_repository.clone(),
        state.user_repository.clone(),
    );

    let input = CreatePostInput {
        title: payload.title,
        description: payload.description,
        author_id: user.id,
        post_type: match payload.post_type.as_str() {
            "photo" => PostType::Photo,
            _ => return Err(AppError::ValidationError(ValidationErrors::new())),
        },
        content_url: payload.content_url,
        visibility: match payload.visibility.as_str() {
            "public" => PostVisibilityType::Public,
            "private" => PostVisibilityType::Private,
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
    like_count: i32,
    comments: Option<Vec<GetPostCommentResponse>>,
    location_id: Option<Uuid>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetPostCommentResponse {
    id: Uuid,
    content: String,
    username: String,
    avatar_url: String,
    user_id: Uuid,
    parent_id: Option<Uuid>,
}

async fn get_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<GetPostResponse>> {
    let post_usecase = GetPostUseCase::new(state.post_repository.clone());
    let get_post_likes_use_case =
        get_post_likes::GetPostLikesUseCase::new(state.post_likes_repository.clone());
    let get_post_comments_use_case =
        GetPostCommentsUseCase::new(state.post_comments_repository.clone());

    let post = post_usecase.execute(GetPostInput { id }).await?;
    let likes = get_post_likes_use_case
        .execute(GetPostLikesInput { id: id })
        .await?;
    let comments = get_post_comments_use_case
        .execute(GetPostCommentsInput { id: id })
        .await?;

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
                PostVisibilityType::Public => "public".into(),
                PostVisibilityType::Private => "private".into(),
            },
            location_id: post.post.location_id.map(|id| id.into()),
            like_count: likes
                .unwrap_or(get_post_likes::GetPostLikesOutput { like_count: 0 })
                .like_count,
            comments: comments.map(|comments| {
                comments
                    .comments
                    .iter()
                    .map(|comment| GetPostCommentResponse {
                        id: comment.0.id.clone().into(),
                        username: comment.1.username.clone(),
                        avatar_url: comment.1.avatar_url.clone().unwrap_or_default(),
                        content: comment.0.content.clone(),
                        user_id: comment.0.clone().user_id.into(),
                        parent_id: comment.0.clone().parent_id.map(|id| id.into()),
                    })
                    .collect()
            }),
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

    if user.id != post.unwrap().post.author_id.id
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
struct UpdatePostRequest {
    title: String,
    description: String,
    post_type: String,
    visibility: String,
    location_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdatePostResponse {
    id: Uuid,
    description: String,
    post_type: String,
    content_url: String,
    author_id: Uuid,
    visibility: String,
    location_id: Option<Uuid>,
    created_at: DateTime<Utc>,
}

async fn update_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<UpdatePostRequest>,
) -> AppResult<Json<UpdatePostResponse>> {
    let post_use_case = GetPostUseCase::new(state.post_repository.clone());
    let update_post_use_case = UpdatePostUseCase::new(state.post_repository.clone());
    let post = post_use_case.execute(GetPostInput { id }).await?;

    if post.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    let unwraped_post = post.unwrap();

    if user.id != unwraped_post.post.author_id.id
        && user.role != UserType::Administrator
        && user.role != UserType::Moderator
    {
        return Err(AppError::Unauthorized("Unauthorized".into()));
    }

    let input = Post {
        id: id.into(),
        title: payload.title,
        description: payload.description,
        author_id: unwraped_post.post.author_id.into(),
        post_type: match payload.post_type.as_str() {
            "photo" => PostType::Photo,
            _ => return Err(AppError::ValidationError(ValidationErrors::new())),
        },
        content_url: unwraped_post.post.content_url,
        visibility: match payload.visibility.as_str() {
            "public" => PostVisibilityType::Public,
            "private" => PostVisibilityType::Private,
            _ => return Err(AppError::ValidationError(ValidationErrors::new())),
        },
        location_id: payload.location_id.map(|id| id.into()),
        created_at: unwraped_post.post.created_at,
    };

    let result = update_post_use_case
        .execute(UpdatePostInput { post: input })
        .await?;

    if result.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    let updated_post = result.unwrap();
    anyhow::Result::Ok(Json(UpdatePostResponse {
        id: updated_post.post.id.into(),
        description: updated_post.post.description,
        post_type: match updated_post.post.post_type {
            PostType::Photo => "photo".into(),
        },
        author_id: updated_post.post.author_id.into(),
        content_url: updated_post.post.content_url,
        visibility: match updated_post.post.visibility {
            PostVisibilityType::Public => "public".into(),
            PostVisibilityType::Private => "private".into(),
        },
        location_id: updated_post.post.location_id.map(|id| id.into()),
        created_at: updated_post.post.created_at,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LikePostResponse {
    success: bool,
}

async fn like_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    user: AuthUser,
) -> AppResult<Json<LikePostResponse>> {
    let like_use_case = LikePostUseCase::new(state.post_likes_repository.clone());

    let result = like_use_case
        .execute(LikePostInput {
            post_id: id,
            user_id: user.id,
        })
        .await?;

    if result.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    anyhow::Result::Ok(Json(LikePostResponse { success: true }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnlikePostResponse {
    success: bool,
}

async fn unlike_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    user: AuthUser,
) -> AppResult<Json<UnlikePostResponse>> {
    let like_use_case = UnlikePostUseCase::new(state.post_likes_repository.clone());

    let result = like_use_case
        .execute(UnlikePostInput {
            post_id: id,
            user_id: user.id,
        })
        .await?;

    if result.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    anyhow::Result::Ok(Json(UnlikePostResponse { success: true }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CheckLikeResponse {
    liked: bool,
    like_count: i32,
}

async fn check_like_get(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    user: AuthUser,
) -> AppResult<Json<CheckLikeResponse>> {
    let liked_by_user_use_case = PostLikedByUserUseCase::new(state.post_likes_repository.clone());
    let get_post_likes_use_case =
        get_post_likes::GetPostLikesUseCase::new(state.post_likes_repository.clone());

    let liked_result = liked_by_user_use_case
        .execute(PostLikedByUserInput {
            post_id: id,
            user_id: user.id,
        })
        .await?;

    let likes_result = get_post_likes_use_case
        .execute(GetPostLikesInput { id: id })
        .await?;

    if liked_result.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    if likes_result.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    anyhow::Result::Ok(Json(CheckLikeResponse {
        like_count: likes_result.unwrap().like_count,
        liked: liked_result.unwrap().liked,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommentPostRequest {
    content: String,
    parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommentPostResponse {
    id: Uuid,
}

async fn comment_post(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<CommentPostRequest>,
) -> AppResult<Json<CommentPostResponse>> {
    let comment_use_case = CommentPostUseCase::new(state.post_comments_repository.clone());

    let result = comment_use_case
        .execute(CommentPostInput {
            post_id: id,
            user_id: user.id,
            content: payload.content,
            parent_id: payload.parent_id,
        })
        .await?;

    if result.is_none() {
        return Err(AppError::NotFound("Comment".into()));
    }

    anyhow::Result::Ok(Json(CommentPostResponse {
        id: result.unwrap().id,
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeletePostCommentResponse {
    success: bool,
}

async fn delete_post_comment(
    state: State<AppState>,
    Path(ids): Path<(Uuid, Uuid)>,
    user: AuthUser,
) -> AppResult<Json<DeletePostCommentResponse>> {
    let uncomment_use_case = UncommentPostUseCase::new(state.post_comments_repository.clone());
    let post_use_case = GetPostUseCase::new(state.post_repository.clone());
    let comment_use_case = GetCommentUseCase::new(state.post_comments_repository.clone());

    let post = post_use_case.execute(GetPostInput { id: ids.0 }).await?;
    let comment = comment_use_case
        .execute(GetCommentInput { id: ids.1 })
        .await?;

    if post.is_none() {
        return Err(AppError::NotFound("Post".into()));
    }

    if comment.is_none() {
        return Err(AppError::NotFound("Comment".into()));
    }

    let unwraped_post = post.unwrap();
    let unwraped_comment = comment.unwrap();

    if user.id != unwraped_post.post.author_id.id
        && user.id != unwraped_comment.comment.user_id.id
        && user.role != UserType::Administrator
        && user.role != UserType::Moderator
    {
        return Err(AppError::Unauthorized("Unauthorized".into()));
    }

    let result = uncomment_use_case
        .execute(UncommentPostInput { id: ids.1 })
        .await?;

    if result.is_none() {
        return Err(AppError::NotFound("Comment".into()));
    }

    anyhow::Result::Ok(Json(DeletePostCommentResponse { success: true }))
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
        .route("/:id", put(update_post))
        .route("/:id/comment", post(comment_post))
        .route("/:id/comment/:comment_id", delete(delete_post_comment))
        .route("/:id/like/check", get(check_like_get))
        .route("/:id/like", post(like_post))
        .route("/:id/like", delete(unlike_post))
        .route("/upload_image", post(upload_image))
}
