use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Location::Table)
                    .add_column(ColumnDef::new(Location::Latitude).not_null().double())
                    .add_column(ColumnDef::new(Location::Longitude).not_null().double())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Location::Table)
                    .drop_column(Location::Latitude)
                    .drop_column(Location::Longitude)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Longitude,
    Latitude,
}
