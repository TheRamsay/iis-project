use std::{future::Future, sync::Arc};

use models::{
    domain::{group::Group, post::Post, post_like::PostLike, post_tag::PostTag, user::User, Id},
    schema,
};
use sea_orm::{
    sea_query::extension::postgres::PgExpr, Condition, DbConn, DbErr, EntityTrait, IntoSimpleExpr,
    PaginatorTrait, QueryFilter,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DbTagRepository {
    db: Arc<DbConn>,
}

impl DbTagRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait TagRepository {
    async fn create(&self, tag: PostTag) -> Result<(String, Uuid), DbErr>;
    async fn delete_by_id(&self, id: Id<PostTag>, tag: &str) -> Result<(), DbErr>;
    async fn search(&self, query: String) -> Result<Option<Vec<PostTag>>, DbErr>;
}

impl TagRepository for DbTagRepository {
    async fn create(&self, tag: PostTag) -> Result<(String, Uuid), DbErr> {
        let post_tag_model: models::schema::post_tag::Model = tag.into();
        let active_model: models::schema::post_tag::ActiveModel = post_tag_model.into();

        let inserted = models::schema::post_tag::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id)
    }

    async fn delete_by_id(&self, id: Id<PostTag>, tag: &str) -> Result<(), DbErr> {
        models::schema::post_tag::Entity::delete_by_id::<(String, Uuid)>((tag.into(), id.id))
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn search(&self, query: String) -> Result<Option<Vec<PostTag>>, DbErr> {
        let Tags = models::schema::post_tag::Entity::find()
            .filter(
                models::schema::post_tag::Column::Tag
                    .into_simple_expr()
                    .ilike(format!("%{}%", query)),
            )
            .all(self.db.as_ref())
            .await?;

        Ok(Some(
            Tags.into_iter().map(PostTag::from).collect::<Vec<_>>(),
        ))
    }
}
