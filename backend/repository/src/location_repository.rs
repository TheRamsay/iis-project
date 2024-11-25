use std::sync::Arc;

use models::domain::{location::Location, Id};
use sea_orm::{
    sea_query::extension::postgres::PgExpr, DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter,
};

#[derive(Debug, Clone)]
pub struct DbLocationRepository {
    db: Arc<DbConn>,
}

impl DbLocationRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait LocationRepository {
    async fn create(&self, like: Location) -> Result<Id<Location>, DbErr>;
    async fn delete_by_id(&self, id: Id<Location>) -> Result<(), DbErr>;
    async fn search(&self, query: String) -> Result<Option<Vec<Location>>, DbErr>;
}

impl LocationRepository for DbLocationRepository {
    async fn create(&self, location: Location) -> Result<Id<Location>, DbErr> {
        let post_location_model: models::schema::location::Model = location.into();
        let active_model: models::schema::location::ActiveModel = post_location_model.into();

        let inserted = models::schema::location::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }

    async fn delete_by_id(&self, id: Id<Location>) -> Result<(), DbErr> {
        models::schema::location::Entity::delete_by_id(id.id)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }

    async fn search(&self, query: String) -> Result<Option<Vec<Location>>, DbErr> {
        let locations = models::schema::location::Entity::find()
            .filter(
                models::schema::location::Column::Name
                    .into_simple_expr()
                    .ilike(format!("%{}%", query)),
            )
            .all(self.db.as_ref())
            .await?;

        Ok(Some(
            locations
                .into_iter()
                .map(Location::from)
                .collect::<Vec<_>>(),
        ))
    }
}
