use models::{
    domain::{
        group_join_request::{GroupJoinRequest, GroupJoinRequestStatus},
        Id,
    },
    errors::AppResult,
};
use repository::{
    group_join_request_repository::GroupJoinRequestRepository,
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct JoinGroupInput {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

pub struct JoinGroupUseCase<
    G: GroupRepository,
    R: GroupJoinRequestRepository,
    M: GroupMemberRepository,
> {
    group_repository: G,
    group_join_request_repository: R,
    group_member_repository: M,
}

impl<G, R, M> JoinGroupUseCase<G, R, M>
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

    pub async fn execute(&self, input: JoinGroupInput) -> AppResult<()> {
        let (group, _) = self
            .group_repository
            .get_by_id(&Id::new(input.group_id))
            .await?
            .ok_or(models::errors::AppError::NotFound(
                "Group not found".to_string(),
            ))?;

        let group_member = self
            .group_member_repository
            .get_by_id(group.id.clone(), input.user_id.into())
            .await?;

        if let Some(_) = group_member {
            return Err(models::errors::AppError::Conflict(
                "User has already joined this group".to_string(),
            ));
        }

        let last_request = self
            .group_join_request_repository
            .find_by_user_id_and_group_id(&input.user_id.into(), &group.id)
            .await?
            .last()
            .cloned();

        if let Some(request) = last_request {
            if request.status == GroupJoinRequestStatus::Pending {
                return Err(models::errors::AppError::Conflict(
                    "User has already requested to join this group".to_string(),
                ));
            }
        }

        let _ = self
            .group_join_request_repository
            .create(GroupJoinRequest::new(group.id, input.user_id.into()))
            .await?;

        Ok(())
    }
}
