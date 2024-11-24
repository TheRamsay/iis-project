use ::chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

use crate::schema;

use super::{post::Post, user::User, Id};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostLike {
    pub post_id: Id<Post>,
    pub user_id: Id<User>,
    pub created_at: DateTime<Utc>,
}

impl PostLike {
    pub fn new(post_id: Id<Post>, user_id: Id<User>) -> Self {
        Self {
            post_id,
            user_id,
            created_at: Utc::now(),
        }
    }
}

impl From<schema::post_like::Model> for PostLike {
    fn from(model: schema::post_like::Model) -> Self {
        Self {
            post_id: Id::new(model.post_id),
            user_id: Id::new(model.user_id),
            created_at: model.created_at.and_utc(),
        }
    }
}

impl From<PostLike> for schema::post_like::ActiveModel {
    fn from(like: PostLike) -> Self {
        Self {
            post_id: ActiveValue::Set(like.post_id.id),
            user_id: ActiveValue::Set(like.user_id.id),
            created_at: ActiveValue::Set(like.created_at.naive_utc()),
        }
    }
}

impl From<PostLike> for schema::post_like::Model {
    fn from(value: PostLike) -> Self {
        Self {
            post_id: value.post_id.id,
            user_id: value.user_id.id,
            created_at: value.created_at.naive_utc(),
        }
    }
}
