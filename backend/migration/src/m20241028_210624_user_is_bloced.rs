use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::IsBlocked).not_null().boolean())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(User::Table)
                    .drop_column(User::IsBlocked)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    IsBlocked,
}
