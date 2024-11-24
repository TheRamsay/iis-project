use std::sync::Arc;

use models::domain::{post::Post, post_like::PostLike, user::User, Id};
use sea_orm::{
    DbConn, DbErr, EntityTrait, IntoSimpleExpr,
    PaginatorTrait, QueryFilter,
};

#[derive(Debug, Clone)]
pub struct DbPostLikesRepository {
    db: Arc<DbConn>,
}

impl DbPostLikesRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait PostLikesRepository {
    async fn create(&self, like: PostLike) -> Result<Id<Post>, DbErr>;
    async fn delete(&self, post_id: Id<Post>, user_id: Id<User>) -> Result<(), DbErr>;
    async fn get_likes_by_id(&self, id: Id<Post>) -> Result<Option<i32>, DbErr>;
}

impl PostLikesRepository for DbPostLikesRepository {
    async fn create(&self, post_like: PostLike) -> Result<Id<Post>, DbErr> {
        let post_like_model: models::schema::post_like::Model = post_like.into();
        let active_model: models::schema::post_like::ActiveModel = post_like_model.into();

        let inserted = models::schema::post_like::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.0.into())
    }

    async fn delete(&self, post_id: Id<Post>, user_id: Id<User>) -> Result<(), DbErr> {
        models::schema::post_like::Entity::delete_by_id((post_id.id, user_id.id))
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn get_likes_by_id(&self, id: Id<Post>) -> Result<Option<i32>, DbErr> {
        let likes = models::schema::post_like::Entity::find()
            .filter(
                models::schema::post_like::Column::PostId
                    .into_simple_expr()
                    .eq(id.id),
            )
            .count(self.db.as_ref())
            .await?;

        Ok(Some(likes as i32))
    }
}
