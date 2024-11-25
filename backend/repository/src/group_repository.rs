use std::sync::Arc;

use models::{
    domain::{group::Group, user::User, Id},
    schema,
};
use sea_orm::{
    sea_query::{extension::postgres::PgExpr, ExprTrait},
    DbConn, DbErr, EntityTrait, IntoSimpleExpr, QueryFilter, Set,
};

#[derive(Debug, Clone)]
pub struct DbGroupRepository {
    db: Arc<DbConn>,
}

impl DbGroupRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait GroupRepository {
    async fn get_by_id(&self, id: &Id<Group>) -> Result<Option<(Group, User)>, DbErr>;
    async fn create(&self, group: Group) -> Result<Id<Group>, DbErr>;
    async fn search(
        &self,
        query: String,
        where_member: Option<Id<User>>,
    ) -> Result<Vec<(Group, User)>, DbErr>;
    async fn delete(&self, group: Id<Group>) -> Result<(), DbErr>;
}

impl GroupRepository for DbGroupRepository {
    async fn get_by_id(&self, id: &Id<Group>) -> Result<Option<(Group, User)>, DbErr> {
        println!("Getting group by id: {:?}", id);
        let result = models::schema::group::Entity::find_by_id(id.id)
            .find_also_related(models::schema::user::Entity)
            .one(self.db.as_ref())
            .await?;

        match result {
            Some((group, author)) => match author {
                Some(author) => Ok(Some((group.into(), author.into()))),
                None => unreachable!("Group without author"),
            },
            _ => Ok(None),
        }
    }

    async fn create(&self, group: Group) -> Result<Id<Group>, DbErr> {
        let group_model: models::schema::group::Model = group.into();
        let active_model: models::schema::group::ActiveModel = group_model.into();

        let inserted = models::schema::group::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }

    async fn search(
        &self,
        query: String,
        where_member: Option<Id<User>>,
    ) -> Result<Vec<(Group, User)>, DbErr> {
        let group_ids = if let Some(ref where_member) = where_member {
            let group_ids = models::schema::group_member::Entity::find()
                .filter(
                    schema::group_member::Column::UserId
                        .into_simple_expr()
                        .eq(where_member.id),
                )
                .all(self.db.as_ref())
                .await?
                .into_iter()
                .map(|group_member| group_member.group_id)
                .collect::<Vec<_>>();

            group_ids
        } else {
            vec![]
        };

        let result = if where_member.is_some() {
            models::schema::group::Entity::find()
                .filter(
                    schema::group::Column::Name
                        .into_simple_expr()
                        .ilike(format!("%{}%", query))
                        .and(
                            schema::group::Column::Id
                                .into_simple_expr()
                                .is_in(group_ids),
                        ),
                )
                .find_also_related(models::schema::user::Entity)
                .all(self.db.as_ref())
                .await?
        } else {
            models::schema::group::Entity::find()
                .filter(
                    schema::group::Column::Name
                        .into_simple_expr()
                        .ilike(format!("%{}%", query)),
                )
                .find_also_related(models::schema::user::Entity)
                .all(self.db.as_ref())
                .await?
        };

        Ok(result
            .into_iter()
            .map(|(group, author)| (group.into(), author.expect("Unknown author").into()))
            .collect())
    }

    async fn delete(&self, group_id: Id<Group>) -> Result<(), DbErr> {
        let active_model = models::schema::group::ActiveModel {
            id: Set(group_id.into()),
            ..Default::default()
        };

        let _ = models::schema::group::Entity::delete(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }
}
