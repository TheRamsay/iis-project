use anyhow::anyhow;
use models::{
    domain::{
        group::Group,
        group_join_request::GroupJoinRequest,
        group_member::GroupMember,
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
use sea_orm::{sqlx, ColIdx, DbErr};
use uuid::Uuid;

#[derive(Debug)]
pub struct AddUserToGroupInput {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub admin_id: Uuid,
}

pub struct AddUserToGroupUseCase<G: GroupRepository, M: GroupMemberRepository> {
    group_repository: G,
    group_member_repository: M,
}

impl<G, M> AddUserToGroupUseCase<G, M>
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

    pub async fn execute(&self, input: AddUserToGroupInput) -> AppResult<()> {
        let (group, admin) = self
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
            .create(GroupMember::new(
                input.user_id.into(),
                input.group_id.into(),
            )?)
            .await
            .map_err(|err| match err {
                DbErr::Exec(sea_orm::RuntimeErr::SqlxError(ref error)) => match error {
                    sqlx::Error::Database(db_error) => {
                        if db_error.code().expect("xk") == "23505" {
                            models::errors::AppError::Conflict(
                                "User has already joined this group".to_string(),
                            )
                        } else {
                            models::errors::AppError::Anyhow(anyhow!(err))
                        }
                    }
                    e => models::errors::AppError::Anyhow(anyhow!(e.to_string())),
                },
                e => models::errors::AppError::Anyhow(anyhow!(e)),
            })?;

        Ok(())
    }
}
