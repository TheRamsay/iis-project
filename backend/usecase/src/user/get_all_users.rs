use models::{
    domain::user::{User, UserType},
    errors::AppResult,
};
use repository::user_repository::UserRepository;

pub struct GetAllUsersInput {
    pub filter_role: Option<UserType>,
    pub filter_is_blocked: Option<bool>,
    pub filter_username: Option<String>,
}

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

    pub async fn execute(&self, input: GetAllUsersInput) -> AppResult<GetAllUsersOutput> {
        let mut data = self.user_repository.get_all().await?;

        if let Some(filter_role) = input.filter_role {
            data = data
                .into_iter()
                .filter(|user| user.user_type == filter_role)
                .collect();
        }

        if let Some(filter_is_blocked) = input.filter_is_blocked {
            data = data
                .into_iter()
                .filter(|user| user.is_blocked == filter_is_blocked)
                .collect();
        }

        if let Some(filter_username) = input.filter_username {
            data = data
                .into_iter()
                .filter(|user| {
                    user.username
                        .to_lowercase()
                        .contains(&filter_username.to_lowercase())
                })
                .collect();
        }

        Ok(data)
    }
}
