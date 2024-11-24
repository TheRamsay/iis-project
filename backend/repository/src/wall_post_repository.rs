use std::sync::Arc;

use models::domain::wall_post::WallPost;
use sea_orm::{DbConn, DbErr, EntityTrait};

#[derive(Debug, Clone)]
pub struct DbWallPostRepository {
    db: Arc<DbConn>,
}

impl DbWallPostRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait WallPostRepository {
    async fn create(&self, wall_post: WallPost) -> Result<(), DbErr>;
}

impl WallPostRepository for DbWallPostRepository {
    async fn create(&self, wall_post: WallPost) -> Result<(), DbErr> {
        let wall_post_model: models::schema::wall_post::Model = wall_post.into();
        let active_model: models::schema::wall_post::ActiveModel = wall_post_model.into();

        let _ = models::schema::wall_post::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }
}
