use chrono::{DateTime, Utc};
use models::{
    domain::{group::Group, user::User, Id},
    errors::{AppError, AppResult},
};
use repository::{
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
};

#[derive(Debug)]
pub struct GetGroupMembersInput {
    pub id: Id<Group>,
}

pub struct GetGroupMembersOutput {
    pub members: Vec<(DateTime<Utc>, User)>,
}

pub struct GetGroupMembersUseCase<T, X>
where
    T: GroupMemberRepository,
    X: GroupRepository,
{
    group_member_repository: T,
    group_repository: X,
}

impl<T, X> GetGroupMembersUseCase<T, X>
where
    T: GroupMemberRepository,
    X: GroupRepository,
{
    pub fn new(group_member_repository: T, group_repository: X) -> Self {
        Self {
            group_member_repository,
            group_repository,
        }
    }

    pub async fn execute(&self, input: GetGroupMembersInput) -> AppResult<GetGroupMembersOutput> {
        self.group_repository
            .get_by_id(&input.id)
            .await?
            .ok_or(AppError::NotFound("Group".to_string()))?;

        let members = self
            .group_member_repository
            .get_by_group_id(input.id)
            .await?;

        Ok(GetGroupMembersOutput { members })
    }
}
