use models::{
    domain::{group::Group, user::User, Id},
    errors::{AppError, AppResult},
};
use repository::{
    group_join_request_repository::GroupJoinRequestRepository,
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct GroupMemberStatusInput {
    pub user_id: Id<User>,
    pub group_id: Id<Group>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupMemberStatus {
    Joined,
    NotJoined,
    Pending,
}

pub struct GroupMemberStatusOutput {
    pub status: GroupMemberStatus,
}

pub struct GroupMemberStatusUseCase<T, X, Y>
where
    T: GroupJoinRequestRepository,
    X: GroupRepository,
    Y: GroupMemberRepository,
{
    group_join_request_repository: T,
    group_repository: X,
    group_member_repository: Y,
}

impl<T, X, Y> GroupMemberStatusUseCase<T, X, Y>
where
    T: GroupJoinRequestRepository,
    X: GroupRepository,
    Y: GroupMemberRepository,
{
    pub fn new(
        group_join_request_repository: T,
        group_repository: X,
        group_member_repository: Y,
    ) -> Self {
        Self {
            group_join_request_repository,
            group_repository,
            group_member_repository,
        }
    }

    pub async fn execute(
        &self,
        input: GroupMemberStatusInput,
    ) -> AppResult<GroupMemberStatusOutput> {
        let (group, _) = self
            .group_repository
            .get_by_id(&input.group_id)
            .await?
            .ok_or(AppError::NotFound("Group".to_string()))?;

        if self
            .group_member_repository
            .get_by_id(input.group_id.clone(), input.user_id.clone())
            .await?
            .is_none()
        {
            return Ok(GroupMemberStatusOutput {
                status: GroupMemberStatus::NotJoined,
            });
        }

        if group.admin_id == input.user_id {
            return Ok(GroupMemberStatusOutput {
                status: GroupMemberStatus::Joined,
            });
        }

        let requests = self
            .group_join_request_repository
            .find_by_user_id_and_group_id(&input.user_id, &input.group_id)
            .await?;

        if requests.is_empty() {
            return Ok(GroupMemberStatusOutput {
                status: GroupMemberStatus::NotJoined,
            });
        }

        let newest_request = requests.first().unwrap();

        match newest_request.status {
            models::domain::group_join_request::GroupJoinRequestStatus::Rejected => {
                Ok(GroupMemberStatusOutput {
                    status: GroupMemberStatus::NotJoined,
                })
            }
            models::domain::group_join_request::GroupJoinRequestStatus::Accepted => {
                Ok(GroupMemberStatusOutput {
                    status: GroupMemberStatus::Joined,
                })
            }
            models::domain::group_join_request::GroupJoinRequestStatus::Pending => {
                Ok(GroupMemberStatusOutput {
                    status: GroupMemberStatus::Pending,
                })
            }
        }
    }
}
