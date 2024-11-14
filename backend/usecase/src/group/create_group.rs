use models::{
    domain::{group::Group, group_member::GroupMember, wall::Wall, Id},
    errors::AppResult,
};
use repository::{
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
    wall_repository::WallRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateGroupInput {
    pub name: String,
    pub admin_id: Uuid,
}

pub struct CreateGroupOutput {
    pub id: Uuid,
}

pub struct CreateGroupUseCase<T, U, H>
where
    T: GroupRepository,
    U: WallRepository,
    H: GroupMemberRepository,
{
    group_repository: T,
    wall_repository: U,
    group_member_repository: H,
}

impl<T, U, H> CreateGroupUseCase<T, U, H>
where
    T: GroupRepository,
    U: WallRepository,
    H: GroupMemberRepository,
{
    pub fn new(group_repository: T, wall_repository: U, group_member_repository: H) -> Self {
        Self {
            group_repository,
            wall_repository,
            group_member_repository,
        }
    }

    pub async fn execute(&self, input: CreateGroupInput) -> AppResult<CreateGroupOutput> {
        let wall_id = self.wall_repository.create(Wall { id: Id::gen() }).await?;

        let group = Group::new(input.name, Id::new(input.admin_id), wall_id)?;
        let group_id = self.group_repository.create(group).await?.id;

        self.group_member_repository
            .create(GroupMember::new(
                Id::new(input.admin_id),
                Id::new(group_id),
            )?)
            .await?;

        Ok(CreateGroupOutput { id: group_id })
    }
}
