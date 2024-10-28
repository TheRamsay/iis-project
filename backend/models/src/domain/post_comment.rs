use validator::{Validate, ValidationErrors};

use crate::schema::{self, post_comment};

use super::{post::Post, user::User, Id};

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct PostComment {
    pub id: Id<PostComment>,
    pub post_id: Id<Post>,
    pub user_id: Id<User>,
    #[validate(length(min = 1, max = 255))]
    pub content: String,
    pub parent_id: Option<Id<PostComment>>,
}

impl PostComment {
    pub fn new(
        post_id: Id<Post>,
        user_id: Id<User>,
        content: String,
        parent_id: Option<Id<PostComment>>,
    ) -> Result<Self, ValidationErrors> {
        let post_comment = Self {
            id: Id::gen(),
            post_id,
            user_id,
            content,
            parent_id,
        };

        post_comment.validate()?;

        Ok(post_comment)
    }
}

impl From<schema::post_comment::Model> for PostComment {
    fn from(model: schema::post_comment::Model) -> Self {
        Self {
            id: Id::new(model.id),
            post_id: Id::new(model.post_id),
            user_id: Id::new(model.user_id),
            content: model.content,
            parent_id: model.parent_id.map(Id::new),
        }
    }
}

impl From<PostComment> for schema::post_comment::Model {
    fn from(model: PostComment) -> Self {
        Self {
            id: model.id.id,
            post_id: model.post_id.id,
            user_id: model.user_id.id,
            content: model.content,
            parent_id: model.parent_id.map(|id| id.id),
        }
    }
}
