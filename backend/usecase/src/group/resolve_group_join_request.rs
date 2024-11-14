use anyhow::anyhow;
use models::{
    domain::{
        group::Group,
        group_join_request::{self, GroupJoinRequest, GroupJoinRequestStatus},
        group_member::GroupMember,
        user::{User, UserType},
        Id,
    },
    errors::{AppError, AppResult},
};
use repository::{
    group_join_request_repository::GroupJoinRequestRepository,
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
    user_repository::UserRepository,
};
use sea_orm::{sqlx, ColIdx, DbErr};
use uuid::Uuid;

#[derive(Debug)]
pub struct ResolveGroupJoinRequestInput {
    pub id: Uuid,
    pub user_id: Uuid,
    pub new_status: GroupJoinRequestStatus,
}

pub struct ResolveGroupJoinRequestUseCase<
    G: GroupRepository,
    R: GroupJoinRequestRepository,
    M: GroupMemberRepository,
> {
    group_repository: G,
    group_join_request_repository: R,
    group_member_repository: M,
}

impl<G, R, M> ResolveGroupJoinRequestUseCase<G, R, M>
where
    G: GroupRepository,
    R: GroupJoinRequestRepository,
    M: GroupMemberRepository,
{
    pub fn new(
        group_repository: G,
        group_join_request_repository: R,
        group_member_repository: M,
    ) -> Self {
        Self {
            group_repository,
            group_join_request_repository,
            group_member_repository,
        }
    }

    pub async fn execute(&self, input: ResolveGroupJoinRequestInput) -> AppResult<()> {
        let mut group_join_request = self
            .group_join_request_repository
            .get_by_id(input.id.into())
            .await?
            .ok_or(AppError::NotFound("Request not found".into()))?;

        let (_, admin) = self
            .group_repository
            .get_by_id(&group_join_request.group_id)
            .await?
            .ok_or(AppError::NotFound("Group not found".into()))?;

        if admin.id.id != input.user_id {
            return Err(AppError::Unauthorized(
                "Only group admin can resolve join requests".into(),
            ));
        }

        match input.new_status {
            GroupJoinRequestStatus::Accepted => {
                let _ = self.group_member_repository.create(GroupMember::new(
                    input.user_id.into(),
                    group_join_request.group_id.clone(),
                )?);

                group_join_request.accept()
            }
            GroupJoinRequestStatus::Rejected => group_join_request.reject(),
            _ => return Err(AppError::BadRequest("Invalid status".into())),
        }

        self.group_join_request_repository
            .update(group_join_request)
            .await?;

        Ok(())
    }
}
