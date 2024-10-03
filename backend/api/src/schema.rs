// use sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
// #[sea_orm(table_name = "users")]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     pub id: uuid::Uuid,
//     pub display_name: String,
//     pub username: String,
//     pub email: String,
//     pub avatar_url: String,
//     pub user_type: UserType,
// }

// #[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "i32", db_type = "Integer")]
// pub enum UserType {
//     Regular = 0,
//     Moderator = 1,
//     Administrator = 2,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

// impl ActiveModelBehavior for ActiveModel {}
