use crate::m20220101_000001_create_user_table::User;
use crate::m20241010_142037_create_group_tables::Group;
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
                    .as_enum(Alias::new("group_join_status_type"))
                    .values(GroupJoinStatusType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GroupJoinRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GroupJoinRequest::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(date_time(GroupJoinRequest::CreatedAt))
                    .col(date_time_null(GroupJoinRequest::ResolvedAt).null())
                    .col(uuid(GroupJoinRequest::GroupId).not_null())
                    .col(uuid(GroupJoinRequest::UserId).not_null())
                    .col(
                        ColumnDef::new(GroupJoinRequest::Status)
                            .enumeration(
                                Alias::new("group_join_status_type"),
                                GroupJoinStatusType::iter(),
                            )
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("group_join_request_group_id_fkey")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .from(GroupJoinRequest::Table, GroupJoinRequest::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("group_join_request_user_id_fkey")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .from(GroupJoinRequest::Table, GroupJoinRequest::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GroupJoinRequest::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("group_join_status_type"))
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
pub enum GroupJoinRequest {
    Table,
    Id,
    GroupId,
    UserId,
    Status,
    CreatedAt,
    ResolvedAt,
}

#[derive(Iden, EnumIter)]
pub enum GroupJoinStatusType {
    Pending,
    Accepted,
    Rejected,
}
