use models::{
    domain::user::{User, UserType},
    errors::AppResult,
};
use repository::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct SearchUserByUsernameInput {
    pub username: String,
}

pub type SearchUserByUsernameOutput = Vec<User>;

pub struct SearchUserByUsernameUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> SearchUserByUsernameUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        input: SearchUserByUsernameInput,
    ) -> AppResult<Option<SearchUserByUsernameOutput>> {
        Ok(self
            .user_repository
            .search_user_by_username(input.username)
            .await?)
    }
}
