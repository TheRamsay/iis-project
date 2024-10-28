use models::{
    domain::{
        group::Group,
        users::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::{group_repository::GroupRepository, user_repository::UserRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetGroupInput {
    pub id: Uuid,
}

pub struct GetGroupOutput {
    pub group: Group,
    pub admin: User,
}

pub struct GetGroupUseCase<T>
where
    T: GroupRepository,
{
    group_repository: T,
}

impl<T> GetGroupUseCase<T>
where
    T: GroupRepository,
{
    pub fn new(group_repository: T) -> Self {
        Self { group_repository }
    }

    pub async fn execute(&self, input: GetGroupInput) -> AppResult<Option<GetGroupOutput>> {
        match self.group_repository.get_by_id(Id::new(input.id)).await? {
            Some((group, admin)) => Ok(Some(GetGroupOutput { group, admin })),
            None => Ok(None),
        }
    }
}
