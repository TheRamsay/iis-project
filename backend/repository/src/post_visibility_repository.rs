use std::{future::Future, sync::Arc};

use models::{
    domain::{
        group::Group, post::Post, post_group_visibility::PostGroupVisibility, post_like::PostLike,
        post_tag::PostTag, post_user_visibility::PostUserVisibility, user::User, Id,
    },
    schema,
};
use sea_orm::{
    sea_query::extension::postgres::PgExpr, Condition, DbConn, DbErr, EntityTrait, IntoSimpleExpr,
    PaginatorTrait, QueryFilter,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DbPostVisibilityRepository {
    db: Arc<DbConn>,
}

impl DbPostVisibilityRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait PostVisibilityRepository {
    async fn create_group_visibility(
        &self,
        visibility: PostGroupVisibility,
    ) -> Result<(Uuid, Uuid), DbErr>;
    async fn create_user_visibility(
        &self,
        visibility: PostUserVisibility,
    ) -> Result<(Uuid, Uuid), DbErr>;
    async fn delete_post_user_visibility(
        &self,
        post_id: Id<Post>,
        user_id: Id<User>,
    ) -> Result<(), DbErr>;
    async fn delete_post_group_visibility(
        &self,
        post_id: Id<Post>,
        group_id: Id<Group>,
    ) -> Result<(), DbErr>;
}

impl PostVisibilityRepository for DbPostVisibilityRepository {
    async fn create_group_visibility(
        &self,
        visibility: PostGroupVisibility,
    ) -> Result<(Uuid, Uuid), DbErr> {
        let post_tag_model: models::schema::post_group_visibility::Model = visibility.into();
        let active_model: models::schema::post_group_visibility::ActiveModel =
            post_tag_model.into();

        let inserted = models::schema::post_group_visibility::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id)
    }

    async fn create_user_visibility(
        &self,
        visibility: PostUserVisibility,
    ) -> Result<(Uuid, Uuid), DbErr> {
        let post_user_visibility_model: models::schema::post_user_visibility::Model =
            visibility.into();
        let active_model: models::schema::post_user_visibility::ActiveModel =
            post_user_visibility_model.into();

        let inserted = models::schema::post_user_visibility::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id)
    }

    async fn delete_post_user_visibility(
        &self,
        post_id: Id<Post>,
        user_id: Id<User>,
    ) -> Result<(), DbErr> {
        models::schema::post_user_visibility::Entity::delete_by_id::<(Uuid, Uuid)>((
            post_id.id, user_id.id,
        ))
        .exec(self.db.as_ref())
        .await?;

        Ok(())
    }

    async fn delete_post_group_visibility(
        &self,
        post_id: Id<Post>,
        group_id: Id<Group>,
    ) -> Result<(), DbErr> {
        models::schema::post_group_visibility::Entity::delete_by_id::<(Uuid, Uuid)>((
            post_id.id,
            group_id.id,
        ))
        .exec(self.db.as_ref())
        .await?;

        Ok(())
    }
}
