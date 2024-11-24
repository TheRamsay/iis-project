use models::{
    domain::{
        group::Group,
        user::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::{group_repository::GroupRepository, user_repository::UserRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct DeleteGroupInput {
    pub id: Id<Group>,
}

pub struct DeleteGroupUseCase<T>
where
    T: GroupRepository,
{
    group_repository: T,
}

impl<T> DeleteGroupUseCase<T>
where
    T: GroupRepository,
{
    pub fn new(group_repository: T) -> Self {
        Self { group_repository }
    }

    pub async fn execute(&self, input: DeleteGroupInput) -> AppResult<()> {
        self.group_repository.delete(input.id).await?;

        Ok(())
    }
}
