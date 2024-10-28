use std::{future::Future, sync::Arc};

use models::domain::{users::User, Id};
use sea_orm::{DbConn, DbErr, EntityTrait};

#[derive(Debug, Clone)]
pub struct DbUserRepository {
    db: Arc<DbConn>,
}

impl DbUserRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait UserRepository {
    async fn get_by_id(&self, id: Id<User>) -> Result<Option<User>, DbErr>;
    async fn create(&self, user: User) -> Result<Id<User>, DbErr>;
}

impl UserRepository for DbUserRepository {
    async fn get_by_id(&self, id: Id<User>) -> Result<Option<User>, DbErr> {
        let user = models::schema::user::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(user.map(User::from))
    }

    async fn create(&self, user: User) -> Result<Id<User>, DbErr> {
        let user_model: models::schema::user::Model = user.into();
        let active_model: models::schema::user::ActiveModel = user_model.into();

        let inserted = models::schema::user::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }
}
