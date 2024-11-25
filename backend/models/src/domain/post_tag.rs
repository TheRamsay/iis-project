use once_cell::sync::Lazy;
use regex::Regex;
use validator::Validate;

use crate::schema;

use super::{post::Post, Id};

static RE_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct PostTag {
    pub post_id: Id<Post>,
    #[validate(length(min = 3, max = 10), regex(path = *RE_TAG, message = "Invalid tag, only alphanumeric characters are allowed",))]
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
