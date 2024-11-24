use models::{
    domain::user::{User, UserType},
    errors::{AppError, AppResult},
};
use repository::user_repository::UserRepository;
use sea_orm::{DbErr, RuntimeErr, SqlxError};
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

use super::auth_utils::hash_password;

#[derive(Debug)]
pub struct UpdateUserInput {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub user_type: UserType,
    pub password: String,
    pub user: User,
}

pub type UpdateUserOutput = User;

pub struct UpdateUserUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> UpdateUserUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: UpdateUserInput) -> AppResult<UpdateUserOutput> {
        let model = User {
            id: input.id.into(),
            email: input.email.clone(),
            username: input.username.clone(),
            display_name: input.display_name,
            avatar_url: input.avatar_url,
            user_type: input.user_type,
            password_hash: hash_password(&input.password)?,
            ..input.user
        };

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
            .get_by_username(input.username.clone())
            .await?
        {
            if u.id != model.id {
                let mut validation_error = ValidationError::new("username");
                validation_error = validation_error.with_message("Username already exists".into());
                validation_error.add_param("value".into(), &input.username);
                validation_errors.add("username", validation_error);
            }
        }

        if input.email.is_some() {
            if let Some(u) = self
                .user_repository
                .get_by_email(input.email.unwrap().clone())
                .await?
            {
                if u.id != model.id {
                    let mut validation_error = ValidationError::new("email");
                    validation_error = validation_error.with_message("Email already exists".into());
                    validation_error.add_param("value".into(), &input.username);
                    validation_errors.add("email", validation_error);
                }
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

        match self.user_repository.update(model).await {
            Ok(user) => Ok(user),
            // Check if unique constraint is violated
            Err(DbErr::Query(RuntimeErr::SqlxError(SqlxError::Database(e))))
                if e.code().unwrap() == "23505" =>
            {
                Err(AppError::Conflict(
                    "Username or email already exists".into(),
                ))
            }
            Err(e) => Err(e.into()),
        }
    }
}
