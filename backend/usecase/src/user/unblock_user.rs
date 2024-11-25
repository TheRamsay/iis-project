use models::errors::{AppError, AppResult};

use repository::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct UnblockUserInput {
    pub user_id: Uuid,
}

pub struct UnblockUserUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> UnblockUserUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: UnblockUserInput) -> AppResult<()> {
        let user = self.user_repository.get_by_id(input.user_id.into()).await?;

        if user.is_none() {
            return Err(AppError::NotFound("User not found".into()));
        }

        let mut user = user.unwrap();

        user.unblock();

        self.user_repository.update(user).await?;

        Ok(())
    }
}
