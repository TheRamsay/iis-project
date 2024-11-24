use models::{
    domain::{
        group::Group,
        group_join_request::{GroupJoinRequest, GroupJoinRequestStatus},
        group_member,
        user::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::{
    group_join_request_repository::GroupJoinRequestRepository,
    group_member_repository::GroupMemberRepository, group_repository::GroupRepository,
    user_repository::UserRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetGroupRequestsInput {
    pub id: Id<Group>,
}

pub struct GetGroupRequestsOutput {
    pub id: Id<GroupJoinRequest>,
    pub user: User,
    pub status: GroupJoinRequestStatus,
}

pub struct GetGroupRequestsUseCase<T>
where
    T: GroupJoinRequestRepository,
{
    group_join_request_repository: T,
}

impl<T> GetGroupRequestsUseCase<T>
where
    T: GroupJoinRequestRepository,
{
    pub fn new(group_join_request_repository: T) -> Self {
        Self {
            group_join_request_repository,
        }
    }

    pub async fn execute(
        &self,
        input: GetGroupRequestsInput,
    ) -> AppResult<Vec<GetGroupRequestsOutput>> {
        let members = self
            .group_join_request_repository
            .find_by_group_id(&input.id)
            .await?;

        Ok(members
            .into_iter()
            .map(|(id, user, status)| GetGroupRequestsOutput { id, user, status })
            .collect())
    }
}
