use models::{
    domain::{group::Group, wall::Wall, Id},
    errors::AppResult,
};
use repository::{group_repository::GroupRepository, wall_repository::WallRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateGroupInput {
    pub name: String,
    pub admin_id: Uuid,
}

pub struct CreateGroupOutput {
    pub id: Uuid,
}

pub struct CreateGroupUseCase<T, U>
where
    T: GroupRepository,
    U: WallRepository,
{
    group_repository: T,
    wall_repository: U,
}

impl<T, U> CreateGroupUseCase<T, U>
where
    T: GroupRepository,
    U: WallRepository,
{
    pub fn new(group_repository: T, wall_repository: U) -> Self {
        Self {
            group_repository,
            wall_repository,
        }
    }

    pub async fn execute(&self, input: CreateGroupInput) -> AppResult<CreateGroupOutput> {
        let wall_id = self.wall_repository.create(Wall { id: Id::gen() }).await?;

        let group = Group::new(input.name, Id::new(input.admin_id), wall_id)?;

        Ok(CreateGroupOutput {
            id: self.group_repository.create(group).await?.id,
        })
    }
}
