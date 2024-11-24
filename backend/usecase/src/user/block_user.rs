use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use models::{
    domain::{
        user::{User, UserType},
        wall::Wall,
        Id,
    },
    errors::{AppError, AppResult},
};

use repository::{user_repository::UserRepository, wall_repository::WallRepository};
use uuid::Uuid;

use super::auth_utils::hash_password;

#[derive(Debug)]
pub struct BlockUserInput {
    pub user_id: Uuid,
}

pub struct BlockUserUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> BlockUserUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: BlockUserInput) -> AppResult<()> {
        let user = self.user_repository.get_by_id(input.user_id.into()).await?;

        if user.is_none() {
            return Err(AppError::NotFound("User not found".into()));
        }

        let mut user = user.unwrap();

        user.block();

        self.user_repository.update(user).await?;

        Ok(())
    }
}
