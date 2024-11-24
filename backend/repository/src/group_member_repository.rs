use std::{future::Future, sync::Arc};

use models::{
    domain::{group::Group, group_member::GroupMember, user::User, Id},
    schema,
};
use sea_orm::{
    sqlx::types::chrono::{self, Utc},
    DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter,
};

#[derive(Debug, Clone)]
pub struct DbGroupMemberRepository {
    db: Arc<DbConn>,
}

impl DbGroupMemberRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait GroupMemberRepository {
    async fn get_by_id(
        &self,
        group_id: Id<Group>,
        user_id: Id<User>,
    ) -> Result<Option<GroupMember>, DbErr>;
    async fn create(&self, group_member: GroupMember) -> Result<(), DbErr>;
    async fn delete(&self, group_member: GroupMember) -> Result<(), DbErr>;
    async fn get_by_group_id(
        &self,
        group_id: Id<Group>,
    ) -> Result<Vec<(chrono::DateTime<Utc>, User)>, DbErr>;
}

impl GroupMemberRepository for DbGroupMemberRepository {
    async fn get_by_id(
        &self,
        group_id: Id<Group>,
        user_id: Id<User>,
    ) -> Result<Option<GroupMember>, DbErr> {
        let result = models::schema::group_member::Entity::find_by_id((user_id.id, group_id.id))
            .one(self.db.as_ref())
            .await?;

        Ok(result.map(|group_member| group_member.into()))
    }

    async fn create(&self, group_member: GroupMember) -> Result<(), DbErr> {
        let group_member_model: models::schema::group_member::Model = group_member.into();
        let active_model: models::schema::group_member::ActiveModel = group_member_model.into();

        let _ = models::schema::group_member::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn delete(&self, group_member: GroupMember) -> Result<(), DbErr> {
        let group_member_model: models::schema::group_member::Model = group_member.into();
        let active_model: models::schema::group_member::ActiveModel = group_member_model.into();

        let _ = models::schema::group_member::Entity::delete(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn get_by_group_id(
        &self,
        group_id: Id<Group>,
    ) -> Result<Vec<(chrono::DateTime<Utc>, User)>, DbErr> {
        let result = models::schema::group_member::Entity::find()
            .filter(
                schema::group_member::Column::GroupId
                    .into_simple_expr()
                    .eq(group_id.id),
            )
            .find_also_related(schema::user::Entity)
            .all(self.db.as_ref())
            .await?;

        Ok(result
            .into_iter()
            .map(|(gm, user)| {
                (
                    gm.joined_at.and_utc(),
                    User::from(user.expect("User not found")),
                )
            })
            .collect())
    }
}
