use anyhow::anyhow;
use models::{
    domain::{
        group::Group,
        group_member::GroupMember,
        user::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::{
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
    user_repository::UserRepository,
};
use sea_orm::{sqlx, ColIdx, DbErr};
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
        let group = self
            .group_repository
            .get_by_id(Id::new(input.group_id))
            .await?;

        if None == group {
            return Err(models::errors::AppError::NotFound(
                "Group not found".to_string(),
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
