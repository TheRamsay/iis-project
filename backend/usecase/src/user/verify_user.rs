use models::errors::{AppError, AppResult};

use repository::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct VerifyUserInput {
    pub username: String,
    pub password: String,
}

pub struct VerifyUserOutput {
    pub id: Uuid,
}

pub struct VerifyUserUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> VerifyUserUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: VerifyUserInput) -> AppResult<VerifyUserOutput> {
        let user = self.user_repository.get_by_username(input.username).await?;

        if user.is_none() {
            return Err(AppError::Unauthorized("Invalid username".to_string()));
        }

        Ok(VerifyUserOutput {
            id: user.unwrap().id.into(),
        })
    }
}
