use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_user_table::User;
use crate::m20241009_204559_create_post_table::Post;
use crate::m20241010_142036_create_wall_tables::{Wall, WallPost};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Group::Id).uuid().not_null().primary_key())
                    .col(string(Group::Name))
                    .col(uuid(Group::AdminId))
                    .col(uuid(Group::WallId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_admin")
                            .from(Group::Table, Group::AdminId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_wall")
                            .from(Group::Table, Group::WallId)
                            .to(Post::Table, Post::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GroupMember::Table)
                    .if_not_exists()
                    .col(uuid(GroupMember::UserId))
                    .col(uuid(GroupMember::GroupId))
                    .primary_key(
                        Index::create()
                            .col(GroupMember::UserId)
                            .col(GroupMember::GroupId),
                    )
                    .col(date_time(GroupMember::JoinedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_member_user")
                            .from(GroupMember::Table, GroupMember::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_member_group")
                            .from(GroupMember::Table, GroupMember::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GroupMember::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    Name,
    AdminId,
    WallId,
}

#[derive(DeriveIden)]
enum GroupMember {
    Table,
    UserId,
    GroupId,
    JoinedAt,
}
