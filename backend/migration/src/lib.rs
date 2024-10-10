pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20241009_204556_create_location_table;
mod m20241009_204559_create_post_table;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20241009_204556_create_location_table::Migration),
            Box::new(m20241009_204559_create_post_table::Migration),
        ]
    }
}
