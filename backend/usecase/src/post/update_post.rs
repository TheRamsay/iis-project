use models::{
    domain::post::Post,
    errors::{AppError, AppResult},
};
use repository::post_repository::PostRepository;

#[derive(Debug)]
pub struct UpdatePostInput {
    pub post: Post,
}

pub struct UpdatePostOutput {
    pub post: Post,
}

pub struct UpdatePostUseCase<T>
where
    T: PostRepository,
{
    post_repository: T,
}

impl<T> UpdatePostUseCase<T>
where
    T: PostRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, input: UpdatePostInput) -> AppResult<Option<UpdatePostOutput>> {
        match self.post_repository.update(input.post).await? {
            Some(post) => Ok(Some(UpdatePostOutput { post })),
            None => Err(AppError::NotFound("Post not found".to_string())),
        }
    }
}
