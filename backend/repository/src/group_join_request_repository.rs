use std::{future::Future, sync::Arc};

use models::{
    domain::{
        group::Group,
        group_join_request::{GroupJoinRequest, GroupJoinRequestStatus},
        user::User,
        Id,
    },
    schema,
};
use sea_orm::{DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter, Set};

#[derive(Debug, Clone)]
pub struct DbGroupJoinRequestRepository {
    db: Arc<DbConn>,
}

impl DbGroupJoinRequestRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait GroupJoinRequestRepository {
    async fn get_by_id(&self, id: Id<GroupJoinRequest>) -> Result<Option<GroupJoinRequest>, DbErr>;
    async fn create(
        &self,
        group_join_request: GroupJoinRequest,
    ) -> Result<Id<GroupJoinRequest>, DbErr>;
    async fn delete(&self, group_join_request: GroupJoinRequest) -> Result<(), DbErr>;
    async fn update(&self, group_join_request: GroupJoinRequest) -> Result<(), DbErr>;
    async fn find_by_user_id_and_group_id(
        &self,
        user_id: &Id<User>,
        group_id: &Id<Group>,
    ) -> Result<Vec<GroupJoinRequest>, DbErr>;
}

impl GroupJoinRequestRepository for DbGroupJoinRequestRepository {
    async fn get_by_id(&self, id: Id<GroupJoinRequest>) -> Result<Option<GroupJoinRequest>, DbErr> {
        let result = models::schema::group_join_request::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(result.map(|x| x.into()))
    }

    async fn create(
        &self,
        group_join_request: GroupJoinRequest,
    ) -> Result<Id<GroupJoinRequest>, DbErr> {
        let group_join_request_model: models::schema::group_join_request::Model =
            group_join_request.into();
        let active_model: models::schema::group_join_request::ActiveModel =
            group_join_request_model.into();

        let inserted = models::schema::group_join_request::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }

    async fn delete(&self, group_join_request: GroupJoinRequest) -> Result<(), DbErr> {
        let group_join_request_model: models::schema::group_join_request::Model =
            group_join_request.into();
        let active_model: models::schema::group_join_request::ActiveModel =
            group_join_request_model.into();

        let _ = models::schema::group_join_request::Entity::delete(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn update(&self, group_join_request: GroupJoinRequest) -> Result<(), DbErr> {
        let group_join_request_model: models::schema::group_join_request::Model =
            group_join_request.into();

        let mut active_model: models::schema::group_join_request::ActiveModel =
            group_join_request_model.into();

        active_model.status = Set(GroupJoinRequestStatus::Accepted.into());

        let _ = models::schema::group_join_request::Entity::update(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn find_by_user_id_and_group_id(
        &self,
        user_id: &Id<User>,
        group_id: &Id<Group>,
    ) -> Result<Vec<GroupJoinRequest>, DbErr> {
        let result = models::schema::group_join_request::Entity::find()
            .filter(
                schema::group_join_request::Column::UserId
                    .into_simple_expr()
                    .eq(user_id.id)
                    .and(
                        schema::group_join_request::Column::GroupId
                            .into_simple_expr()
                            .eq(group_id.id),
                    ),
            )
            .all(self.db.as_ref())
            .await?;

        Ok(result.into_iter().map(|x| x.into()).collect())
    }
}
