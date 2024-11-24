use crate::schema;

use super::{post::Post, Id};

#[derive(Clone, Debug, PartialEq)]
pub struct PostTag {
    pub post_id: Id<Post>,
    pub tag: String,
}

impl PostTag {
    pub fn new(post_id: Id<Post>, tag: String) -> Self {
        Self { post_id, tag }
    }
}

impl From<schema::post_tag::Model> for PostTag {
    fn from(model: schema::post_tag::Model) -> Self {
        Self {
            post_id: Id::new(model.post_id),
            tag: model.tag,
        }
    }
}

impl From<PostTag> for schema::post_tag::Model {
    fn from(model: PostTag) -> Self {
        Self {
            post_id: model.post_id.id,
            tag: model.tag,
        }
    }
}
