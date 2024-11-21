use models::{
    domain::user::{User, UserType},
    errors::AppResult,
};
use repository::user_repository::UserRepository;
use uuid::Uuid;
use validator::Validate;

use super::auth_utils::hash_password;

#[derive(Debug)]
pub struct UpdateUserInput {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: String,
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
            email: input.email.try_into()?,
            username: input.username,
            display_name: input.display_name,
            avatar_url: input.avatar_url,
            user_type: input.user_type,
            password_hash: hash_password(&input.password)?,
            ..input.user
        };

        model.validate()?;

        let updated = self.user_repository.update(model).await?;

        Ok(updated)
    }
}
