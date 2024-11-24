use crate::schema;

use super::{group::Group, post::Post, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct PostGroupVisibility {
    pub post_id: Id<Post>,
    pub group_id: Id<Group>,
}

impl PostGroupVisibility {
    pub fn new(post_id: Id<Post>, group_id: Id<Group>) -> Self {
        Self { post_id, group_id }
    }
}

impl From<schema::post_group_visibility::Model> for PostGroupVisibility {
    fn from(model: schema::post_group_visibility::Model) -> Self {
        Self {
            post_id: Id::new(model.post_id),
            group_id: Id::new(model.group_id),
        }
    }
}

impl From<PostGroupVisibility> for schema::post_group_visibility::Model {
    fn from(model: PostGroupVisibility) -> Self {
        Self {
            post_id: model.post_id.id,
            group_id: model.group_id.id,
        }
    }
}
