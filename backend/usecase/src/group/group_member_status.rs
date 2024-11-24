use models::{
    domain::{group::Group, user::User, Id},
    errors::{AppError, AppResult},
};
use repository::{
    group_join_request_repository::GroupJoinRequestRepository, group_repository::GroupRepository,
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

pub struct GroupMemberStatusUseCase<T, X>
where
    T: GroupJoinRequestRepository,
    X: GroupRepository,
{
    group_join_request_repository: T,
    group_repository: X,
}

impl<T, X> GroupMemberStatusUseCase<T, X>
where
    T: GroupJoinRequestRepository,
    X: GroupRepository,
{
    pub fn new(group_join_request_repository: T, group_repository: X) -> Self {
        Self {
            group_join_request_repository,
            group_repository,
        }
    }

    pub async fn execute(
        &self,
        input: GroupMemberStatusInput,
    ) -> AppResult<GroupMemberStatusOutput> {
        self.group_repository
            .get_by_id(&input.group_id)
            .await?
            .ok_or(AppError::NotFound("Group".to_string()))?;

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
