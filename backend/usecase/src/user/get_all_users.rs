use models::{domain::user::User, errors::AppResult};
use repository::user_repository::UserRepository;

pub type GetAllUsersOutput = Vec<User>;

pub struct GetAllUsersUseCase<T>
where
    T: UserRepository,
{
    user_repository: T,
}

impl<T> GetAllUsersUseCase<T>
where
    T: UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self) -> AppResult<GetAllUsersOutput> {
        Ok(self.user_repository.get_all().await?)
    }
}
