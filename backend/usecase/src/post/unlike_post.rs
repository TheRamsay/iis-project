use models::{
    domain::Id,
    errors::AppResult,
};
use repository::post_likes_repository::PostLikesRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct UnlikePostInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

pub struct UnlikePostOutput {}

pub struct UnlikePostUseCase<T>
where
    T: PostLikesRepository,
{
    post_repository: T,
}

impl<T> UnlikePostUseCase<T>
where
    T: PostLikesRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, input: UnlikePostInput) -> AppResult<Option<UnlikePostOutput>> {
        let result = self
            .post_repository
            .delete(Id::new(input.post_id), Id::new(input.user_id))
            .await;

        match result {
            Ok(_) => Ok(Some(UnlikePostOutput {})),
            Err(_) => return Err(anyhow::anyhow!("Failed to unlike post").into()),
        }
    }
}
