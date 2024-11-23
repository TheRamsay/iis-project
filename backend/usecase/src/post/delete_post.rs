use models::{
    domain::{
        post::Post,
        user::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::post_repository::PostRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct DeletePostInput {
    pub id: Uuid,
}

pub struct DeletePostOutput {
    pub success: bool,
}

pub struct DeletePostUseCase<T>
where
    T: PostRepository,
{
    post_repository: T,
}

impl<T> DeletePostUseCase<T>
where
    T: PostRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, input: DeletePostInput) -> AppResult<Option<DeletePostOutput>> {
        match self.post_repository.delete_by_id(Id::new(input.id)).await? {
            true => Ok(Some(DeletePostOutput { success: true })),
            false => Ok(Some(DeletePostOutput { success: false })),
        }
    }
}
