use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_like::PostLike,
        Id,
    },
    errors::AppResult,
};
use repository::{post_likes_repository::PostLikesRepository, post_repository::PostRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct PostLikedByUserInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

pub struct PostLikedByUserOutput {
    pub liked: bool,
}

pub struct PostLikedByUserUseCase<T>
where
    T: PostLikesRepository,
{
    post_likes_repository: T,
}

impl<T> PostLikedByUserUseCase<T>
where
    T: PostLikesRepository,
{
    pub fn new(post_likes_repository: T) -> Self {
        Self {
            post_likes_repository,
        }
    }

    pub async fn execute(
        &self,
        input: PostLikedByUserInput,
    ) -> AppResult<Option<PostLikedByUserOutput>> {
        let liked = self
            .post_likes_repository
            .get_is_liked_by_user(Id::new(input.post_id), Id::new(input.user_id))
            .await?;

        Ok(Some(PostLikedByUserOutput { liked }))
    }
}
