use models::{
    domain::{group_member::GroupMember, Id},
    errors::AppResult,
};
use repository::{
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct RemoveUserToGroupInput {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub admin_id: Uuid,
}

pub struct RemoveUserToGroupUseCase<G: GroupRepository, M: GroupMemberRepository> {
    group_repository: G,
    group_member_repository: M,
}

impl<G, M> RemoveUserToGroupUseCase<G, M>
where
    G: GroupRepository,
    M: GroupMemberRepository,
{
    pub fn new(group_repository: G, group_member_repository: M) -> Self {
        Self {
            group_repository,
            group_member_repository,
        }
    }

    pub async fn execute(&self, input: RemoveUserToGroupInput) -> AppResult<()> {
        let (_, admin) = self
            .group_repository
            .get_by_id(&Id::new(input.group_id))
            .await?
            .ok_or(models::errors::AppError::NotFound(
                "Group not found".to_string(),
            ))?;

        if admin.id.id != input.admin_id {
            return Err(models::errors::AppError::Unauthorized(
                "Only group admin can add user to group".to_string(),
            ));
        }
        self.group_member_repository
            .get_by_id(input.group_id.into(), input.user_id.into())
            .await?
            .ok_or(models::errors::AppError::NotFound(
                "User not found in group".to_string(),
            ))?;

        self.group_member_repository
            .delete(GroupMember::new(
                input.user_id.into(),
                input.group_id.into(),
            )?)
            .await?;

        Ok(())
    }
}
