use models::{
    domain::{group_member::GroupMember, Id},
    errors::AppResult,
};
use repository::{
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct LeaveGroupInput {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

pub struct LeaveGroupUseCase<T, U>
where
    T: GroupRepository,
    U: GroupMemberRepository,
{
    group_repository: T,
    group_member_repository: U,
}

impl<T, U> LeaveGroupUseCase<T, U>
where
    T: GroupRepository,
    U: GroupMemberRepository,
{
    pub fn new(group_repository: T, group_member_repository: U) -> Self {
        Self {
            group_repository,
            group_member_repository,
        }
    }

    pub async fn execute(&self, input: LeaveGroupInput) -> AppResult<()> {
        let (group, _) = self
            .group_repository
            .get_by_id(&Id::new(input.group_id))
            .await?
            .ok_or(models::errors::AppError::NotFound(
                "Group not found".to_string(),
            ))?;

        if group.admin_id.id == input.user_id {
            return Err(models::errors::AppError::BadRequest(
                "Admin cannot leave group".to_string(),
            ));
        }

        let group_member = self
            .group_member_repository
            .get_by_id(Id::new(input.group_id), Id::new(input.user_id))
            .await?;

        if group_member.is_none() {
            return Err(models::errors::AppError::NotFound(
                "User is not a member of the group".to_string(),
            ));
        }

        self.group_member_repository
            .delete(GroupMember::new(
                Id::new(input.user_id),
                Id::new(input.group_id),
            )?)
            .await?;

        Ok(())
    }
}
