use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_user_table::User;
use crate::m20241010_142036_create_wall_tables::Wall;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::WallId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_user_wall")
                            .from_col(User::WallId)
                            .from_tbl(User::Table)
                            .to_col(Wall::Id)
                            .to_tbl(Wall::Table),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_foreign_key(Alias::new("fk_user_wall"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::WallId)
                    .to_owned(),
            )
            .await
    }
}
