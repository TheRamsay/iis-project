use models::{
    domain::user::User,
    errors::AppResult,
};
use repository::user_repository::UserRepository;

#[derive(Debug)]
pub struct GetUserByUsernameInput {
    pub username: String,
}

pub type GetUserByUsernameOutput = User;

pub struct GetUserByUsernameUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> GetUserByUsernameUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        input: GetUserByUsernameInput,
    ) -> AppResult<Option<GetUserByUsernameOutput>> {
        Ok(self.user_repository.get_by_username(input.username).await?)
    }
}
