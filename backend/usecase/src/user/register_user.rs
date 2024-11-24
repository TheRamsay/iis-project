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
use validator::{Validate, ValidationError, ValidationErrors};

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
        let wall_id = self.wall_repository.create(Wall { id: Id::gen() }).await?;

        let model = User::new(
            input.display_name,
            input.username,
            input.email,
            input.avatar_url,
            input.user_type,
            wall_id,
            hash_password(&input.password)?,
        )?;

        let mut validation_errors = ValidationErrors::new();

        // Validate the model for domain rules
        match model.validate() {
            Ok(_) => (),
            Err(e) => {
                validation_errors = e;
            }
        }

        // Check for unique constraints
        if let Some(u) = self
            .user_repository
            .get_by_username(model.username.clone())
            .await?
        {
            if u.id != model.id {
                let mut validation_error = ValidationError::new("username");
                validation_error = validation_error.with_message("Username already exists".into());
                validation_error.add_param("value".into(), &model.username);
                validation_errors.add("username", validation_error);
            }
        }

        if let Some(u) = self
            .user_repository
            .get_by_email(model.email.clone())
            .await?
        {
            if u.id != model.id {
                let mut validation_error = ValidationError::new("email");
                validation_error = validation_error.with_message("Email already exists".into());
                validation_error.add_param("value".into(), &model.username);
                validation_errors.add("email", validation_error);
            }
        }

        if input.password.len() < 3 || input.password.len() > 15 {
            let mut validation_error = ValidationError::new("password");
            validation_error = validation_error
                .with_message("Password must be between 3 and 15 characters".into());
            validation_error.add_param("value".into(), &input.password);
            validation_errors.add("password", validation_error);
        }

        if !validation_errors.is_empty() {
            return Err(validation_errors.into());
        }
        Ok(RegisterUserOutput {
            id: self.user_repository.create(model).await?.id,
        })
    }
}
