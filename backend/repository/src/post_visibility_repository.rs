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
    async fn get_post_group_visibilities(
        &self,
        post_id: Id<Post>,
    ) -> Result<Vec<(PostGroupVisibility, Group)>, DbErr>;
    async fn get_post_user_visibilities(
        &self,
        post_id: Id<Post>,
    ) -> Result<Vec<(PostUserVisibility, User)>, DbErr>;
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

    async fn get_post_group_visibilities(
        &self,
        post_id: Id<Post>,
    ) -> Result<Vec<(PostGroupVisibility, Group)>, DbErr> {
        let group_visibilities = models::schema::post_group_visibility::Entity::find()
            .filter(
                models::schema::post_group_visibility::Column::PostId
                    .into_simple_expr()
                    .eq(post_id.id),
            )
            .all(self.db.as_ref())
            .await?;

        let mut result = Vec::new();

        for visibility in group_visibilities.into_iter() {
            let group = models::schema::group::Entity::find_by_id(visibility.group_id)
                .one(self.db.as_ref())
                .await?;

            if let Some(group) = group {
                result.push((visibility.into(), group.into()));
            }
        }

        Ok(result)
    }

    async fn get_post_user_visibilities(
        &self,
        post_id: Id<Post>,
    ) -> Result<Vec<(PostUserVisibility, User)>, DbErr> {
        let user_visibilities = models::schema::post_user_visibility::Entity::find()
            .filter(
                models::schema::post_user_visibility::Column::PostId
                    .into_simple_expr()
                    .eq(post_id.id),
            )
            .all(self.db.as_ref())
            .await?;

        let mut result = Vec::new();

        for visibility in user_visibilities.into_iter() {
            let user = models::schema::user::Entity::find_by_id(visibility.user_id)
                .one(self.db.as_ref())
                .await?;

            if let Some(user) = user {
                result.push((visibility.into(), user.into()));
            }
        }

        Ok(result)
    }
}
