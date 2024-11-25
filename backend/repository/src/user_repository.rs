use models::domain::{user::User, Id};
use sea_orm::{
    sea_query::extension::postgres::PgExpr, DbConn, DbErr, EntityTrait, IntoSimpleExpr,
    QueryFilter, Set,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DbUserRepository {
    db: Arc<DbConn>,
}

impl DbUserRepository {
    pub fn new(db: Arc<DbConn>) -> Self {
        Self { db }
    }
}

pub trait UserRepository {
    async fn get_all(&self) -> Result<Vec<User>, DbErr>;
    async fn get_by_id(&self, id: Id<User>) -> Result<Option<User>, DbErr>;
    async fn get_by_username(&self, username: String) -> Result<Option<User>, DbErr>;
    async fn search_user_by_username(&self, username: String) -> Result<Option<Vec<User>>, DbErr>;
    async fn get_by_email(&self, email: String) -> Result<Option<User>, DbErr>;
    async fn create(&self, user: User) -> Result<Id<User>, DbErr>;
    async fn update(&self, user: User) -> Result<User, DbErr>;
    async fn delete(&self, user: Id<User>) -> Result<(), DbErr>;
}

impl UserRepository for DbUserRepository {
    async fn get_all(&self) -> Result<Vec<User>, DbErr> {
        let users = models::schema::user::Entity::find()
            .all(self.db.as_ref())
            .await?;

        Ok(users.into_iter().map(User::from).collect())
    }

    async fn get_by_id(&self, id: Id<User>) -> Result<Option<User>, DbErr> {
        let user = models::schema::user::Entity::find_by_id(id.id)
            .one(self.db.as_ref())
            .await?;

        Ok(user.map(User::from))
    }

    async fn create(&self, user: User) -> Result<Id<User>, DbErr> {
        let user_model: models::schema::user::Model = user.into();
        let active_model: models::schema::user::ActiveModel = user_model.into();

        let inserted = models::schema::user::Entity::insert(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(inserted.last_insert_id.into())
    }

    async fn get_by_username(&self, username: String) -> Result<Option<User>, DbErr> {
        let user = models::schema::user::Entity::find()
            .filter(
                models::schema::user::Column::Username
                    .into_simple_expr()
                    .eq(username),
            )
            .one(self.db.as_ref())
            .await?;

        Ok(user.map(User::from))
    }

    async fn search_user_by_username(&self, username: String) -> Result<Option<Vec<User>>, DbErr> {
        let users = models::schema::user::Entity::find()
            .filter(
                models::schema::user::Column::Username
                    .into_simple_expr()
                    .ilike(format!("%{}%", username)),
            )
            .all(self.db.as_ref())
            .await?;

        Ok(Some(users.into_iter().map(User::from).collect()))
    }

    async fn get_by_email(&self, email: String) -> Result<Option<User>, DbErr> {
        let user = models::schema::user::Entity::find()
            .filter(
                models::schema::user::Column::Email
                    .into_simple_expr()
                    .eq(email),
            )
            .one(self.db.as_ref())
            .await?;

        Ok(user.map(User::from))
    }

    async fn update(&self, user: User) -> Result<User, DbErr> {
        let user_member_model: models::schema::user::Model = user.clone().into();
        let mut active_model: models::schema::user::ActiveModel = user_member_model.into();

        active_model.is_blocked = Set(user.is_blocked);
        active_model.avatar_url = Set(user.avatar_url);
        active_model.display_name = Set(user.display_name);
        active_model.email = Set(user.email);
        active_model.user_type = Set(user.user_type.into());
        active_model.username = Set(user.username);
        active_model.password_hash = Set(user.password_hash);

        let updated = models::schema::user::Entity::update(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(updated.into())
    }

    async fn delete(&self, user: Id<User>) -> Result<(), DbErr> {
        let active_model = models::schema::user::ActiveModel {
            id: Set(user.id),
            ..Default::default()
        };

        let _ = models::schema::user::Entity::delete(active_model)
            .exec(self.db.as_ref())
            .await?;

        Ok(())
    }
}
