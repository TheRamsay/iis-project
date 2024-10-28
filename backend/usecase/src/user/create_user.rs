use models::{
    domain::{
        users::{User, UserType},
        wall::Wall,
        Id,
    },
    errors::AppResult,
};
use repository::{user_repository::UserRepository, wall_repository::WallRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateUserInput {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub user_type: UserType,
}

pub struct CreateUserOutput {
    pub id: Uuid,
}

pub struct CreateUserUseCase<T, U>
where
    T: UserRepository,
    U: WallRepository,
{
    user_repository: T,
    wall_repository: U,
}

impl<T, U> CreateUserUseCase<T, U>
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

    pub async fn execute(&self, input: CreateUserInput) -> AppResult<CreateUserOutput> {
        let wall_id = self.wall_repository.create(Wall { id: Id::gen() }).await?;

        let user = User::new(
            input.display_name,
            input.username,
            input.email.try_into()?,
            input.avatar_url,
            input.user_type,
            wall_id,
        )?;

        Ok(CreateUserOutput {
            id: self.user_repository.create(user).await?.id,
        })
    }
}
