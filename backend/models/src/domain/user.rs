use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::schema;

use super::{email::Email, wall::Wall, Id};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserType {
    Administrator,
    Moderator,
    Regular,
}

#[derive(Clone, Debug, PartialEq, Validate, Serialize, Deserialize)]
pub struct User {
    pub id: Id<User>,
    #[validate(length(
        min = 3,
        max = 255,
        message = "Display name must be between 3 and 255 characters"
    ))]
    pub display_name: String,
    #[validate(length(
        min = 3,
        max = 255,
        message = "Username must be between 3 and 255 characters"
    ))]
    pub username: String,
    pub email: Email,
    #[validate(url)]
    pub avatar_url: Option<String>,
    pub user_type: UserType,
    pub wall_id: Id<Wall>,
    pub is_blocked: bool,
    pub password_hash: String,
}

impl User {
    pub fn new(
        display_name: String,
        username: String,
        email: Email,
        avatar_url: Option<String>,
        user_type: UserType,
        wall_id: Id<Wall>,
        password_hash: String,
    ) -> Result<Self, ValidationErrors> {
        let model = Self {
            id: Id::gen(),
            display_name,
            username,
            email,
            avatar_url,
            user_type,
            wall_id,
            is_blocked: false,
            password_hash,
        };

        model.validate()?;

        Ok(model)
    }
}

impl From<schema::user::Model> for User {
    fn from(model: schema::user::Model) -> Self {
        Self {
            id: Id::new(model.id),
            display_name: model.display_name,
            username: model.username,
            email: Email::new(model.email).expect("Invalid email from database"),
            avatar_url: model.avatar_url,
            user_type: match model.user_type {
                schema::sea_orm_active_enums::UserType::Administrator => UserType::Administrator,
                schema::sea_orm_active_enums::UserType::Moderator => UserType::Moderator,
                schema::sea_orm_active_enums::UserType::Regular => UserType::Regular,
            },
            wall_id: Id::new(model.wall_id),
            is_blocked: model.is_blocked,
            password_hash: model.password_hash,
        }
    }
}

impl From<User> for schema::user::Model {
    fn from(user: User) -> Self {
        Self {
            id: user.id.id,
            display_name: user.display_name,
            username: user.username,
            email: user.email.value,
            avatar_url: user.avatar_url,
            user_type: match user.user_type {
                UserType::Administrator => schema::sea_orm_active_enums::UserType::Administrator,
                UserType::Moderator => schema::sea_orm_active_enums::UserType::Moderator,
                UserType::Regular => schema::sea_orm_active_enums::UserType::Regular,
            },
            wall_id: user.wall_id.id,
            is_blocked: user.is_blocked,
            password_hash: user.password_hash,
        }
    }
}
