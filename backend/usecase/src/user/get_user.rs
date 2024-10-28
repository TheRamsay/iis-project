use models::{
    domain::users::{User, UserType},
    errors::AppResult,
};
use repository::user_repository::UserRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct GetUserInput {
    pub id: Uuid,
}

pub type GetUserOutput = User;

pub struct GetUserUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> GetUserUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, input: GetUserInput) -> AppResult<Option<GetUserOutput>> {
        Ok(self.user_repository.get_by_id(input.id.into()).await?)
    }
}
