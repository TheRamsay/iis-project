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
pub struct LikePostInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

pub struct LikePostOutput {
    pub id: Uuid,
}

pub struct LikePostUseCase<T>
where
    T: PostLikesRepository,
{
    post_likes_repository: T,
}

impl<T> LikePostUseCase<T>
where
    T: PostLikesRepository,
{
    pub fn new(post_likes_repository: T) -> Self {
        Self {
            post_likes_repository,
        }
    }

    pub async fn execute(&self, input: LikePostInput) -> AppResult<Option<LikePostOutput>> {
        let post_like = PostLike::new(Id::new(input.post_id), Id::new(input.user_id));

        Ok(Some(LikePostOutput {
            id: self.post_likes_repository.create(post_like).await?.id,
        }))
    }
}
