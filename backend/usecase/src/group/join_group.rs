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
pub struct JoinGroupInput {
    pub user_id: Uuid,
    pub group_id: Uuid,
}

pub struct JoinGroupUseCase<T, U>
where
    T: GroupRepository,
    U: GroupMemberRepository,
{
    group_repository: T,
    group_member_repository: U,
}

impl<T, U> JoinGroupUseCase<T, U>
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

    pub async fn execute(&self, input: JoinGroupInput) -> AppResult<()> {
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
            .create(GroupMember::new(
                input.user_id.into(),
                input.group_id.into(),
            )?)
            .await
            .map_err(|err| { println!("MMMM {:?}", err); match err {
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
            }})?;

        Ok(())
    }
}
