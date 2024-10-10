use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_user_table::User;
use crate::m20241009_204556_create_location_table::Location;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("post_type"))
                    .values(PostType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).uuid().not_null().primary_key())
                    .col(string(Post::Title))
                    .col(string(Post::Description))
                    .col(uuid(Post::LocationId))
                    .col(uuid(Post::AuthorId))
                    .col(date_time(Post::CreatedAt))
                    .col(string(Post::ContentType))
                    .col(string(Post::ContentUrl))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_location")
                            .from(Post::Table, Post::LocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_author")
                            .from(Post::Table, Post::AuthorId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        // manager
        //     .drop_foreign_key(ForeignKey::drop().name("fk_post_location").to_owned())
        //     .await?;

        // manager
        //     .drop_foreign_key(ForeignKey::drop().name("fk_post_author").to_owned())
        //     .await?;

        manager
            .drop_type(Type::drop().name(Alias::new("post_type")).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Description,
    LocationId,
    AuthorId,
    CreatedAt,
    ContentType,
    ContentUrl,
}

#[derive(Iden, EnumIter)]
pub enum PostType {
    #[iden = "image"]
    Image,
}
