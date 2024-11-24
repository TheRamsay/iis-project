use std::sync::Arc;

use models::domain::{
        post::Post, post_comment::PostComment, Id,
    };
use sea_orm::{
    DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter,
};

#[derive(Debug, Clone)]
pub struct DbPostCommentsRepository {
    db: Arc<DbConn>,
}

impl DbPostCommentsRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait PostCommentsRepository {
    async fn create(&self, like: PostComment) -> Result<Id<PostComment>, DbErr>;
    async fn delete_by_id(&self, id: Id<PostComment>) -> Result<(), DbErr>;
    async fn get_comments_by_post_id(
        &self,
        id: Id<Post>,
    ) -> Result<Option<Vec<PostComment>>, DbErr>;
    async fn get_comment_by_id(&self, id: Id<PostComment>) -> Result<Option<PostComment>, DbErr>;
}

impl PostCommentsRepository for DbPostCommentsRepository {
    async fn create(&self, comment: PostComment) -> Result<Id<PostComment>, DbErr> {
        let post_comment_model: models::schema::post_comment::Model = comment.into();
        let active_model: models::schema::post_comment::ActiveModel = post_comment_model.into();

        let inserted = models::schema::post_comment::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }

    async fn delete_by_id(&self, id: Id<PostComment>) -> Result<(), DbErr> {
        models::schema::post_comment::Entity::delete_by_id(id.id)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn get_comments_by_post_id(
        &self,
        id: Id<Post>,
    ) -> Result<Option<Vec<PostComment>>, DbErr> {
        let comments = models::schema::post_comment::Entity::find()
            .filter(
                models::schema::post_comment::Column::PostId
                    .into_simple_expr()
                    .eq(id.id),
            )
            .all(self.db.as_ref())
            .await?;

        let comments: Vec<PostComment> = comments.into_iter().map(|model| model.into()).collect();
        Ok(Some(comments))
    }

    async fn get_comment_by_id(&self, id: Id<PostComment>) -> Result<Option<PostComment>, DbErr> {
        let comment = models::schema::post_comment::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(comment.map(|model| model.into()))
    }
}
