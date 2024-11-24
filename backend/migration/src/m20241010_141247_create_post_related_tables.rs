use crate::m20220101_000001_create_user_table::User;
use crate::m20241009_204559_create_post_table::Post;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostComment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostComment::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(uuid(PostComment::PostId))
                    .col(uuid(PostComment::UserId))
                    .col(string(PostComment::Content))
                    .col(ColumnDef::new(PostComment::ParentId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_comment_post")
                            .from(PostComment::Table, PostComment::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_comment_user")
                            .from(PostComment::Table, PostComment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_comment_parent")
                            .from(PostComment::Table, PostComment::ParentId)
                            .to(PostComment::Table, PostComment::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostLike::Table)
                    .if_not_exists()
                    .col(uuid(PostLike::PostId).not_null())
                    .col(uuid(PostLike::UserId).not_null())
                    .primary_key(Index::create().col(PostLike::PostId).col(PostLike::UserId))
                    .col(date_time(PostLike::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_like_post")
                            .from(PostLike::Table, PostLike::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_like_user")
                            .from(PostLike::Table, PostLike::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    .col(string(PostTag::Tag).not_null().primary_key())
                    .col(uuid(PostTag::PostId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tag_post")
                            .from(PostTag::Table, PostTag::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostVisibility::Table)
                    .if_not_exists()
                    .col(uuid(PostVisibility::PostId))
                    .col(uuid(PostVisibility::UserId))
                    .primary_key(
                        Index::create()
                            .col(PostVisibility::PostId)
                            .col(PostVisibility::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_visibility_post")
                            .from(PostVisibility::Table, PostVisibility::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_visibility_user")
                            .from(PostVisibility::Table, PostVisibility::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostComment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PostLike::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PostTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PostVisibility::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostComment {
    Table,
    Id,
    PostId,
    UserId,
    Content,
    ParentId,
}

#[derive(DeriveIden)]
enum PostLike {
    Table,
    PostId,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    Tag,
    PostId,
}

#[derive(DeriveIden)]
pub enum PostVisibility {
    Table,
    PostId,
    UserId,
}
