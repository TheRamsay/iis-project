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
pub struct RegisterUserInput {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub user_type: UserType,
    pub password: String,
}

pub struct RegisterUserOutput {
    pub id: Uuid,
}

pub struct RegisterUserUseCase<T, U>
where
    T: UserRepository,
    U: WallRepository,
{
    user_repository: T,
    wall_repository: U,
}

impl<T, U> RegisterUserUseCase<T, U>
where
    T: UserRepository,
    U: WallRepository,
{
    pub fn new(user_repository: T, wall_repository: U) -> Self {
        Self {
            user_repository,
            wall_repository,
        }
    }

    pub async fn execute(&self, input: RegisterUserInput) -> AppResult<RegisterUserOutput> {
        if let Some(u) = self
            .user_repository
            .get_by_username(input.username.clone())
            .await?
        {
            return Err(AppError::Conflict(format!(
                "User with username {} already exists",
                u.username
            )));
        }

        if let Some(u) = self
            .user_repository
            .get_by_email(input.email.clone())
            .await?
        {
            return Err(AppError::Conflict(format!(
                "User with email {} already exists",
                u.email
            )));
        }

        let wall_id = self.wall_repository.create(Wall { id: Id::gen() }).await?;

        let user = User::new(
            input.display_name,
            input.username,
            input.email,
            input.avatar_url,
            input.user_type,
            wall_id,
            hash_password(&input.password)?,
        )?;

        Ok(RegisterUserOutput {
            id: self.user_repository.create(user).await?.id,
        })
    }
}
