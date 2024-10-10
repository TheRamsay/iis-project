use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241009_204559_create_post_table::Post;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wall::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Wall::Id).uuid().not_null().primary_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WallPost::Table)
                    .if_not_exists()
                    .col(uuid(WallPost::WallId))
                    .col(uuid(WallPost::PostId))
                    .primary_key(Index::create().col(WallPost::WallId).col(WallPost::PostId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wall_post_wall")
                            .from(WallPost::Table, WallPost::WallId)
                            .to(Wall::Table, Wall::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_wall_post_post")
                            .from(WallPost::Table, WallPost::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WallPost::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Wall::Table).to_owned())
            .await

    }
}

#[derive(DeriveIden)]
pub enum Wall {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum WallPost {
    Table,
    WallId,
    PostId,
}
