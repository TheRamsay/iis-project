use std::{future::Future, sync::Arc};

use models::domain::{
    post::{Post, PostVisibilityType},
    Id,
};
use sea_orm::{
    sea_query::{extension::postgres::PgExpr, ExprTrait},
    DbConn, DbErr, EntityTrait, IntoSimpleExpr, PaginatorTrait, QueryFilter, Set,
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
    async fn delete_by_id(&self, id: Id<Post>) -> Result<bool, DbErr>;
    async fn update(&self, post: Post) -> Result<Option<Post>, DbErr>;
}

impl PostRepository for DbPostRepository {
    async fn get_by_id(&self, id: Id<Post>) -> Result<Option<Post>, DbErr> {
        let post = models::schema::post::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(post.map(Post::from))
    }

    async fn update(&self, post: Post) -> Result<Option<Post>, DbErr> {
        let post_model: models::schema::post::Model = post.clone().into();
        let mut active_model: models::schema::post::ActiveModel = post_model.into();

        active_model.description = Set(post.description);
        active_model.title = Set(post.title);
        active_model.visibility = Set(match post.visibility {
            PostVisibilityType::Public => "public".to_owned(),
            PostVisibilityType::Private => "private".to_owned(),
        });
        active_model.location_id = Set(post.location_id.map(|id| id.id));

        let post = models::schema::post::Entity::update(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(Some(Post::from(post)))
    }

    async fn delete_by_id(&self, id: Id<Post>) -> Result<bool, DbErr> {
        let result = models::schema::post::Entity::delete_by_id(id.id)
            .exec(self.db.as_ref())
            .await?;

        match result.rows_affected {
            1 => Ok(true),
            _ => Ok(false),
        }
    }

    async fn create(&self, post: Post) -> Result<Id<Post>, DbErr> {
        let post_model: models::schema::post::Model = post.into();
        let active_model: models::schema::post::ActiveModel = post_model.into();

        let inserted = models::schema::post::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }
}
