use models::{
    domain::{
        group::Group,
        group_member,
        user::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::{
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
    user_repository::UserRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetGroupMembersInput {
    pub id: Id<Group>,
}

pub struct GetGroupMembersOutput {
    pub members: Vec<User>,
}

pub struct GetGroupMembersUseCase<T>
where
    T: GroupMemberRepository,
{
    group_member_repository: T,
}

impl<T> GetGroupMembersUseCase<T>
where
    T: GroupMemberRepository,
{
    pub fn new(group_member_repository: T) -> Self {
        Self {
            group_member_repository,
        }
    }

    pub async fn execute(&self, input: GetGroupMembersInput) -> AppResult<GetGroupMembersOutput> {
        let members = self
            .group_member_repository
            .get_by_group_id(input.id)
            .await?;

        Ok(GetGroupMembersOutput { members })
    }
}
