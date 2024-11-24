use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241009_204559_create_post_table::Post,
    m20241010_141247_create_post_related_tables::PostVisibility,
    m20241010_142037_create_group_tables::Group,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .rename_table(
                Table::rename()
                    .table(PostVisibility::Table, PostUserVisibility::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostGroupVisibility::Table)
                    .if_not_exists()
                    .col(uuid(PostGroupVisibility::PostId))
                    .col(uuid(PostGroupVisibility::GroupId))
                    .primary_key(
                        Index::create()
                            .col(PostGroupVisibility::PostId)
                            .col(PostGroupVisibility::GroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_group_visibility_post")
                            .from(PostGroupVisibility::Table, PostGroupVisibility::PostId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to(Post::Table, Post::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_group_visibility_group")
                            .from(PostGroupVisibility::Table, PostGroupVisibility::GroupId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostGroupVisibility::Table).to_owned())
            .await?;

        manager
            .rename_table(
                Table::rename()
                    .table(PostUserVisibility::Table, PostVisibility::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PostUserVisibility {
    Table,
}

#[derive(DeriveIden)]
enum PostGroupVisibility {
    Table,
    PostId,
    GroupId,
}
