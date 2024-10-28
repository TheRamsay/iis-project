use crate::schema;

use super::{post::Post, user::User, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct PostVisibility {
    pub post_id: Id<Post>,
    pub user_id: Id<User>,
}

impl PostVisibility {
    pub fn new(post_id: Id<Post>, user_id: Id<User>) -> Self {
        Self { post_id, user_id }
    }
}

impl From<schema::post_visibility::Model> for PostVisibility {
    fn from(model: schema::post_visibility::Model) -> Self {
        Self {
            post_id: Id::new(model.post_id),
            user_id: Id::new(model.user_id),
        }
    }
}

impl From<PostVisibility> for schema::post_visibility::Model {
    fn from(model: PostVisibility) -> Self {
        Self {
            post_id: model.post_id.id,
            user_id: model.user_id.id,
        }
    }
}
