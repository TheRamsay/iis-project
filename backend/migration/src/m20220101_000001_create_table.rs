use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("user_type"))
                    .values(UserType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(string(User::DisplayName))
                    .col(string(User::Username))
                    .col(string(User::Email))
                    .col(string(User::AvatarUrl))
                    .col(
                        ColumnDef::new(User::UserType)
                            .enumeration(Alias::new("user_type"), UserType::iter())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Alias::new("user_type")).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    DisplayName,
    Username,
    Email,
    AvatarUrl,
    UserType,
}

#[derive(Iden, EnumIter)]
pub enum UserType {
    Regular,
    Moderator,
    Administrator,
}
