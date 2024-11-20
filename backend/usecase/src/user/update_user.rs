use models::{
    domain::user::{User, UserType},
    errors::AppResult,
};
use repository::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct UpdateUserInput {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub user_type: UserType,
    pub password: String,
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

    pub async fn execute(&self, input: UpdateUserInput) -> AppResult<Option<UpdateUserOutput>> {

        
    }
}
