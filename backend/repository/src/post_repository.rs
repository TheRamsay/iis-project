use std::{future::Future, sync::Arc};

use models::domain::{post::Post, Id};
use sea_orm::{
    sea_query::{extension::postgres::PgExpr, ExprTrait},
    DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter,
};

#[derive(Debug, Clone)]
pub struct DbPostRepository {
    db: Arc<DbConn>,
}

impl DbPostRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait PostRepository {
    async fn get_by_id(&self, id: Id<Post>) -> Result<Option<Post>, DbErr>;
    async fn create(&self, post: Post) -> Result<Id<Post>, DbErr>;
}

impl PostRepository for DbPostRepository {
    async fn get_by_id(&self, id: Id<Post>) -> Result<Option<Post>, DbErr> {
        let user = models::schema::post::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(user.map(Post::from))
    }

    async fn create(&self, post: Post) -> Result<Id<Post>, DbErr> {
        let user_model: models::schema::post::Model = post.into();
        let active_model: models::schema::post::ActiveModel = user_model.into();

        let inserted = models::schema::post::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }
}
