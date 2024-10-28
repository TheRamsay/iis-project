use sea_orm::sqlx::types::chrono;

use crate::schema;

use super::{post::Post, user::User, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct PostLike {
    pub post_id: Id<Post>,
    pub user_id: Id<User>,
    pub created_at: chrono::NaiveDateTime,
}

impl PostLike {
    pub fn new(post_id: Id<Post>, user_id: Id<User>) -> Self {
        Self {
            post_id,
            user_id,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<schema::post_like::Model> for PostLike {
    fn from(model: schema::post_like::Model) -> Self {
        Self {
            post_id: Id::new(model.post_id),
            user_id: Id::new(model.user_id),
            created_at: model.created_at,
        }
    }
}
