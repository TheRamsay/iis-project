use crate::schema;

use super::{post::Post, user::User, wall::Wall, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct WallPost {
    pub post_id: Id<Post>,
    pub wall_id: Id<Wall>,
}

impl WallPost {
    pub fn new(post_id: Id<Post>, wall_id: Id<Wall>) -> Self {
        Self { post_id, wall_id }
    }
}

impl From<schema::wall_post::Model> for WallPost {
    fn from(model: schema::wall_post::Model) -> Self {
        Self {
            post_id: Id::new(model.post_id),
            wall_id: Id::new(model.wall_id),
        }
    }
}

impl From<WallPost> for schema::wall_post::Model {
    fn from(model: WallPost) -> Self {
        Self {
            post_id: model.post_id.id,
            wall_id: model.wall_id.id,
        }
    }
}
