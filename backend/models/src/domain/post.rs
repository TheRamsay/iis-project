use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::schema;

use super::{user::User, wall::Wall, Id};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum PostType {
    #[serde(rename = "photo")]
    Photo,
}

impl Display for PostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostType::Photo => write!(f, "photo"),
        }
    }
}

impl From<String> for PostType {
    fn from(post_type: String) -> Self {
        match post_type.as_str() {
            "photo" => Self::Photo,
            _ => Self::Photo,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum PostVisibilityType {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Display for PostVisibilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostVisibilityType::Public => write!(f, "public"),
            PostVisibilityType::Private => write!(f, "private"),
        }
    }
}

impl From<String> for PostVisibilityType {
    fn from(visibility: String) -> Self {
        match visibility.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Public,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Validate, Deserialize, Serialize)]
pub struct Post {
    pub id: Id<Post>,
    #[validate(length(
        min = 3,
        max = 500,
        message = "Description must be between 3 and 500 characters"
    ))]
    pub description: String,
    #[validate(length(
        min = 3,
        max = 15,
        message = "Title must be between 3 and 15 characters"
    ))]
    pub title: String,
    pub author_id: Id<User>,
    pub post_type: PostType,
    #[validate(url)]
    pub content_url: String,
    pub visibility: PostVisibilityType,
    pub location_id: Option<Id<Wall>>,
    pub created_at: DateTime<Utc>,
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
            description,
            author_id,
            title,
            post_type,
            content_url,
            visibility,
            location_id,
            created_at: Utc::now(),
        };

        model.validate()?;

        Ok(model)
    }
}

impl From<schema::post::Model> for Post {
    fn from(model: schema::post::Model) -> Self {
        Self {
            id: Id::new(model.id),
            description: model.description,
            author_id: Id::new(model.author_id),
            title: model.title,
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
            created_at: model.created_at.and_utc(),
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
            created_at: value.created_at.naive_utc(),
        }
    }
}
