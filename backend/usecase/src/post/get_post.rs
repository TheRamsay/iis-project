use models::{
    domain::{post::Post, Id},
    errors::AppResult,
};
use repository::post_repository::PostRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct GetPostInput {
    pub id: Uuid,
}

#[derive(Clone)]
pub struct GetPostOutput {
    pub post: Post,
}

pub struct GetPostUseCase<T>
where
    T: PostRepository,
{
    post_repository: T,
}

impl<T> GetPostUseCase<T>
where
    T: PostRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, input: GetPostInput) -> AppResult<Option<GetPostOutput>> {
        match self.post_repository.get_by_id(Id::new(input.id)).await? {
            Some(post) => Ok(Some(GetPostOutput { post })),
            None => Ok(None),
        }
    }
}
