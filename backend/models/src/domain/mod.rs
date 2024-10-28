use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{ValidationError, ValidationErrors};

pub mod email;
pub mod group;
pub mod group_member;
pub mod location;
pub mod post;
pub mod post_comment;
pub mod post_like;
pub mod post_tag;
pub mod post_visibility;
pub mod users;
pub mod wall;
pub mod wall_post;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Id<T> {
    pub id: Uuid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn gen() -> Self {
        Self::new(Uuid::new_v4())
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Self {
        Self::new(id)
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(id: Id<T>) -> Uuid {
        id.id
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = ValidationErrors;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        let value = Uuid::parse_str(&id);

        match value {
            Ok(id) => Ok(Self::new(id)),
            Err(_) => {
                let mut errors = ValidationErrors::new();
                errors.add("id", ValidationError::new("Invalid UUID format"));
                Err(errors)
            }
        }
    }
}
