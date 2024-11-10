use sea_orm::sqlx::types::chrono;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::schema;

use super::{email::Email, user::User, wall::Wall, Id};

#[derive(Debug, Clone, PartialEq)]
pub enum PostType {
    Photo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostVisibilityType {
    Public,
    Private,
}

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct Post {
    pub id: Id<Post>,
    #[validate(length(min = 3, max = 50))]
    pub title: String,
    #[validate(length(min = 3, max = 500))]
    pub description: String,
    pub author_id: Id<User>,
    pub post_type: PostType,
    #[validate(url)]
    pub content_url: String,
    pub visibility: PostVisibilityType,
    pub location_id: Option<Id<Wall>>,
    pub created_at: chrono::NaiveDateTime,
}

impl Post {
    pub fn new(
        title: String,
        description: String,
        author_id: Id<User>,
        post_type: PostType,
        content_url: String,
        visibility: PostVisibilityType,
        location_id: Option<Id<Wall>>,
    ) -> Result<Self, ValidationErrors> {
        let model = Self {
            id: Id::gen(),
            title,
            description,
            author_id,
            post_type,
            content_url,
            visibility,
            location_id,
            created_at: chrono::Utc::now().naive_utc(),
        };

        model.validate()?;

        Ok(model)
    }
}

impl From<schema::post::Model> for Post {
    fn from(model: schema::post::Model) -> Self {
        Self {
            id: Id::new(model.id),
            title: model.title,
            description: model.description,
            author_id: Id::new(model.author_id),
            visibility: match model.visibility.as_str() {
                "public" => PostVisibilityType::Public,
                "private" => PostVisibilityType::Private,
                _ => unreachable!("Invalid visibility type received from database"),
            },
            content_url: model.content_url,
            post_type: match model.content_type.as_str() {
                "photo" => PostType::Photo,
                _ => unreachable!("Invalid post type received from database"),
            },
            location_id: model.location_id.map(|id| Id::new(id)),
            created_at: model.created_at,
        }
    }
}

impl From<Post> for schema::post::Model {
    fn from(value: Post) -> Self {
        Self {
            id: value.id.id,
            title: value.title,
            description: value.description,
            author_id: value.author_id.id,
            visibility: match value.visibility {
                PostVisibilityType::Public => "public".to_string(),
                PostVisibilityType::Private => "private".to_string(),
            },
            content_url: value.content_url,
            content_type: match value.post_type {
                PostType::Photo => "photo".to_string(),
            },
            location_id: value.location_id.map(|id| id.id),
            created_at: value.created_at,
        }
    }
}
