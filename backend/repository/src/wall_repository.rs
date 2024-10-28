use std::{future::Future, sync::Arc};

use models::domain::{users::User, wall::Wall, Id};
use sea_orm::{DbConn, DbErr, EntityTrait};

#[derive(Debug, Clone)]
pub struct DbWallRepository {
    db: Arc<DbConn>,
}

impl DbWallRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait WallRepository {
    async fn get_by_id(&self, id: Id<Wall>) -> Result<Option<Wall>, DbErr>;
    async fn create(&self, wall: Wall) -> Result<Id<Wall>, DbErr>;
}

impl WallRepository for DbWallRepository {
    async fn get_by_id(&self, id: Id<Wall>) -> Result<Option<Wall>, DbErr> {
        let wall = models::schema::wall::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(wall.map(Wall::from))
    }

    async fn create(&self, Wall: Wall) -> Result<Id<Wall>, DbErr> {
        let wall_model: models::schema::wall::Model = Wall.into();
        let active_model: models::schema::wall::ActiveModel = wall_model.into();

        let inserted = models::schema::wall::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }
}
