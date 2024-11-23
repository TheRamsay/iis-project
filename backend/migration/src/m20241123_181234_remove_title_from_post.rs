use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241009_204559_create_post_table::Post;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Post::Table)
                    .drop_column(Post::Title)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Post::Table)
                    .add_column(ColumnDef::new(Post::Title).string())
                    .to_owned(),
            )
            .await
    }
}
