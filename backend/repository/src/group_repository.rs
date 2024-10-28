use std::{future::Future, sync::Arc};

use models::{
    domain::{group::Group, user::User, Id},
    schema,
};
use sea_orm::{DbConn, DbErr, EntityTrait};

#[derive(Debug, Clone)]
pub struct DbGroupRepository {
    db: Arc<DbConn>,
}

impl DbGroupRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait GroupRepository {
    async fn get_by_id(&self, id: Id<Group>) -> Result<Option<(Group, User)>, DbErr>;
    async fn create(&self, group: Group) -> Result<Id<Group>, DbErr>;
}

impl GroupRepository for DbGroupRepository {
    async fn get_by_id(&self, id: Id<Group>) -> Result<Option<(Group, User)>, DbErr> {
        let result = models::schema::group::Entity::find_by_id(id.id)
            .find_also_related(models::schema::user::Entity)
            .one(self.db.as_ref())
            .await?;

        match result {
            Some((group, author)) => match author {
                Some(author) => Ok(Some((group.into(), author.into()))),
                None => unreachable!("Group without author"),
            },
            _ => Ok(None),
        }
    }

    async fn create(&self, group: Group) -> Result<Id<Group>, DbErr> {
        let group_model: models::schema::group::Model = group.into();
        let active_model: models::schema::group::ActiveModel = group_model.into();

        let inserted = models::schema::group::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }
}
