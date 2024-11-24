use axum::{
    extract::{Path, Query, State},
    routing::get,
};
use chrono::{DateTime, Utc};
use models::{
    domain::{
        post::{PostType, PostVisibilityType},
        user::UserType,
    },
    errors::AppResult,
};
use serde::{Deserialize, Serialize};
use usecase::wall::{
    get_feed_posts::{GetFeedPostsInput, GetFeedPostsUseCase},
    get_tag_posts::{GetTagPostsInput, GetTagPostsUseCase},
    get_wall_posts::{GetWallPostsInput, GetWallPostsUseCase},
    types::SortBy,
};
use uuid::Uuid;

use crate::{
    extractors::{auth_extractor::OptionalAuthUser, json_extractor::Json},
    pagination::{self, PaginationParams},
    AppState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthorResponse {
    id: Uuid,
    display_name: Option<String>,
    username: String,
    avatar_url: Option<String>,
    user_type: UserType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub post_type: PostType,
    pub content_url: String,
    pub visibility: PostVisibilityType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostCommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user: GetAuthorResponse,
    pub content: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostLikeResponse {
    post_id: Uuid,
    user: GetAuthorResponse,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostItem {
    post: GetPostResponse,
    author: GetAuthorResponse,
    comments: Vec<GetPostCommentResponse>,
    likes: Vec<GetPostLikeResponse>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWallResponse {
    posts: Vec<PostItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortQuery {
    sort_by: Option<SortBy>,
}

pub async fn get_wall(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
    Query(sort_by): Query<SortQuery>,
) -> AppResult<Json<GetWallResponse>> {
    let get_wall_posts_usecase = GetWallPostsUseCase::new(state.wall_repository.clone());
    let pagination = pagination::Pagination::from(pagination);

    let input = GetWallPostsInput {
        id: id.into(),
        pagination: (pagination.offset, pagination.limit),
        sort_by: sort_by.sort_by.unwrap_or_default(),
    };

    let output = get_wall_posts_usecase.execute(input).await?;

    Ok(Json(GetWallResponse {
        posts: output
            .into_iter()
            .map(|(post, author, comments, likes, tags)| PostItem {
                post: GetPostResponse {
                    id: post.id.into(),
                    title: post.title,
                    description: post.description,
                    post_type: post.post_type,
                    content_url: post.content_url,
                    visibility: post.visibility,
                    created_at: post.created_at,
                },
                author: GetAuthorResponse {
                    id: author.id.into(),
                    display_name: author.display_name,
                    username: author.username,
                    avatar_url: author.avatar_url,
                    user_type: author.user_type,
                },
                comments: comments
                    .into_iter()
                    .map(|(comment, user)| GetPostCommentResponse {
                        id: comment.id.into(),
                        post_id: comment.post_id.into(),
                        content: comment.content,
                        user: GetAuthorResponse {
                            id: user.id.into(),
                            display_name: user.display_name,
                            username: user.username,
                            avatar_url: user.avatar_url,
                            user_type: user.user_type,
                        },
                        parent_id: comment.parent_id.map(|id| id.into()),
                    })
                    .collect(),
                likes: likes
                    .into_iter()
                    .map(|(like, user)| GetPostLikeResponse {
                        post_id: like.post_id.into(),
                        user: GetAuthorResponse {
                            id: user.id.into(),
                            display_name: user.display_name,
                            username: user.username,
                            avatar_url: user.avatar_url,
                            user_type: user.user_type,
                        },
                        created_at: like.created_at,
                    })
                    .collect(),
                tags,
            })
            .collect(),
    }))
}

pub async fn get_feed(
    state: State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Query(pagination): Query<PaginationParams>,
    Query(sort_by): Query<SortQuery>,
) -> AppResult<Json<GetWallResponse>> {
    let get_feed_usecase = GetFeedPostsUseCase::new(state.wall_repository.clone());
    let pagination = pagination::Pagination::from(pagination);

    let input = GetFeedPostsInput {
        user_id: if user.is_some() {
            Some(user.unwrap().id.into())
        } else {
            None
        },
        pagination: (pagination.offset, pagination.limit),
        sort_by: sort_by.sort_by.unwrap_or_default(),
    };

    let output = get_feed_usecase.execute(input).await?;

    Ok(Json(GetWallResponse {
        posts: output
            .into_iter()
            .map(|(post, author, comments, likes, tags)| PostItem {
                post: GetPostResponse {
                    id: post.id.into(),
                    title: post.title,
                    description: post.description,
                    post_type: post.post_type,
                    content_url: post.content_url,
                    visibility: post.visibility,
                    created_at: post.created_at,
                },
                author: GetAuthorResponse {
                    id: author.id.into(),
                    display_name: author.display_name,
                    username: author.username,
                    avatar_url: author.avatar_url,
                    user_type: author.user_type,
                },
                comments: comments
                    .into_iter()
                    .map(|(comment, user)| GetPostCommentResponse {
                        id: comment.id.into(),
                        post_id: comment.post_id.into(),
                        content: comment.content,
                        user: GetAuthorResponse {
                            id: user.id.into(),
                            display_name: user.display_name,
                            username: user.username,
                            avatar_url: user.avatar_url,
                            user_type: user.user_type,
                        },
                        parent_id: comment.parent_id.map(|id| id.into()),
                    })
                    .collect(),
                likes: likes
                    .into_iter()
                    .map(|(like, user)| GetPostLikeResponse {
                        post_id: like.post_id.into(),
                        user: GetAuthorResponse {
                            id: user.id.into(),
                            display_name: user.display_name,
                            username: user.username,
                            avatar_url: user.avatar_url,
                            user_type: user.user_type,
                        },
                        created_at: like.created_at,
                    })
                    .collect(),
                tags,
            })
            .collect(),
    }))
}

pub async fn get_wall_by_tag(
    state: State<AppState>,
    OptionalAuthUser(user): OptionalAuthUser,
    Path(tag_name): Path<String>,
    Query(pagination): Query<PaginationParams>,
    Query(sort_by): Query<SortQuery>,
) -> AppResult<Json<GetWallResponse>> {
    let get_tag_usecase = GetTagPostsUseCase::new(state.wall_repository.clone());

    let pagination = pagination::Pagination::from(pagination);

    let input = GetTagPostsInput {
        user_id: if user.is_some() {
            Some(user.unwrap().id.into())
        } else {
            None
        },
        tag: tag_name,
        pagination: (pagination.offset, pagination.limit),
        sort_by: sort_by.sort_by.unwrap_or_default(),
    };

    let output = get_tag_usecase.execute(input).await?;

    Ok(Json(GetWallResponse {
        posts: output
            .into_iter()
            .map(|(post, author, comments, likes, tags)| PostItem {
                post: GetPostResponse {
                    id: post.id.into(),
                    title: post.title,
                    description: post.description,
                    post_type: post.post_type,
                    content_url: post.content_url,
                    visibility: post.visibility,
                    created_at: post.created_at,
                },
                author: GetAuthorResponse {
                    id: author.id.into(),
                    display_name: author.display_name,
                    username: author.username,
                    avatar_url: author.avatar_url,
                    user_type: author.user_type,
                },
                comments: comments
                    .into_iter()
                    .map(|(comment, user)| GetPostCommentResponse {
                        id: comment.id.into(),
                        post_id: comment.post_id.into(),
                        content: comment.content,
                        user: GetAuthorResponse {
                            id: user.id.into(),
                            display_name: user.display_name,
                            username: user.username,
                            avatar_url: user.avatar_url,
                            user_type: user.user_type,
                        },
                        parent_id: comment.parent_id.map(|id| id.into()),
                    })
                    .collect(),
                likes: likes
                    .into_iter()
                    .map(|(like, user)| GetPostLikeResponse {
                        post_id: like.post_id.into(),
                        user: GetAuthorResponse {
                            id: user.id.into(),
                            display_name: user.display_name,
                            username: user.username,
                            avatar_url: user.avatar_url,
                            user_type: user.user_type,
                        },
                        created_at: like.created_at,
                    })
                    .collect(),
                tags,
            })
            .collect(),
    }))
}

pub fn wall_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/:id", get(get_wall))
        .route("/feed", get(get_feed))
        .route("/tag/:tag_name", get(get_wall_by_tag))
}
