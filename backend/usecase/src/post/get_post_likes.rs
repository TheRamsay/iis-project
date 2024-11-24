use models::{
    domain::Id,
    errors::AppResult,
};
use repository::post_likes_repository::PostLikesRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct GetPostLikesInput {
    pub id: Uuid,
}

pub struct GetPostLikesOutput {
    pub like_count: i32,
}

pub struct GetPostLikesUseCase<T>
where
    T: PostLikesRepository,
{
    post_repository: T,
}

impl<T> GetPostLikesUseCase<T>
where
    T: PostLikesRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, input: GetPostLikesInput) -> AppResult<Option<GetPostLikesOutput>> {
        let result = self
            .post_repository
            .get_likes_by_id(Id::new(input.id))
            .await?;

        match result {
            Some(likes) => Ok(Some(GetPostLikesOutput { like_count: likes })),
            None => Ok(None),
        }
    }
}
