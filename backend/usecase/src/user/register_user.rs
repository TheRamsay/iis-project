use models::{
    domain::{
        user::{User, UserType},
        wall::Wall,
        Id,
    },
    errors::AppResult,
};

use repository::{user_repository::UserRepository, wall_repository::WallRepository};
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use super::auth_utils::hash_password;

#[derive(Debug)]
pub struct RegisterUserInput {
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub username: String,
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

        let model_result = User::new(
            None,
            input.username.clone(),
            input.email.clone(),
            input.avatar_url,
            input.user_type,
            wall_id,
            hash_password(&input.password)?,
        );

        let mut validation_errors = ValidationErrors::new();

        // Validate the model for domain rules
        match model_result {
            Ok(_) => (),
            Err(ref e) => validation_errors = e.clone(),
        }

        // Check for unique constraints
        if let Some(u) = self
            .user_repository
            .get_by_username(input.username.clone())
            .await?
        {
            let mut validation_error = ValidationError::new("username");
            validation_error = validation_error.with_message("Username already exists".into());
            validation_error.add_param("value".into(), &input.username);
            validation_errors.add("username", validation_error);
        }

        if input.email.is_some() {
            if let Some(u) = self
                .user_repository
                .get_by_email(input.email.unwrap().clone())
                .await?
            {
                let mut validation_error = ValidationError::new("email");
                validation_error = validation_error.with_message("Email already exists".into());
                validation_error.add_param("value".into(), &input.username);
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
            id: self.user_repository.create(model_result.unwrap()).await?.id,
        })
    }
}
