use std::{cmp::Ordering, default, fmt::Display, ops};

use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;
use uuid::Uuid;
use validator::{Validate, ValidateArgs, ValidationError, ValidationErrors};

use crate::schema;

use super::{
    email::{self, Email},
    wall::Wall,
    Id,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum UserType {
    #[default]
    #[serde(rename = "regular")]
    Regular = 0,
    #[serde(rename = "moderator")]
    Moderator = 1,
    #[serde(rename = "administrator")]
    Administrator = 2,
}

impl Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Regular => write!(f, "regular"),
            UserType::Moderator => write!(f, "moderator"),
            UserType::Administrator => write!(f, "administrator"),
        }
    }
}

impl From<String> for UserType {
    fn from(user_type: String) -> Self {
        match user_type.as_str() {
            "regular" => Self::Regular,
            "moderator" => Self::Moderator,
            "administrator" => Self::Administrator,
            _ => Self::Regular,
        }
    }
}

impl UserType {
    pub fn has_higher_privilege_than(&self, other: &UserType) -> bool {
        (self.to_owned() as i32) > (other.to_owned() as i32)
    }

    pub fn has_lower_privilege_than(&self, other: &UserType) -> bool {
        (self.to_owned() as i32) < (other.to_owned() as i32)
    }

    pub fn has_same_privilege_as(&self, other: &UserType) -> bool {
        (self.to_owned() as i32) == (other.to_owned() as i32)
    }

    pub fn is_administrator(&self) -> bool {
        self.has_same_privilege_as(&UserType::Administrator)
    }

    pub fn is_moderator(&self) -> bool {
        self.has_same_privilege_as(&UserType::Moderator)
    }

    pub fn is_regular(&self) -> bool {
        self.has_same_privilege_as(&UserType::Regular)
    }

    pub fn has_highter_or_same_privilege_as(&self, other: &UserType) -> bool {
        self.has_higher_privilege_than(other) || self.has_same_privilege_as(other)
    }

    pub fn has_lower_or_same_privilege_as(&self, other: &UserType) -> bool {
        self.has_lower_privilege_than(other) || self.has_same_privilege_as(other)
    }
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
    #[validate(email)]
    pub email: String,
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
        email: String,
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

    pub fn block(&mut self) {
        self.is_blocked = true;
    }
}

impl From<schema::user::Model> for User {
    fn from(model: schema::user::Model) -> Self {
        Self {
            id: Id::new(model.id),
            display_name: model.display_name,
            username: model.username,
            email: model.email,
            avatar_url: model.avatar_url,
            user_type: model.user_type.into(),
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
            email: user.email,
            avatar_url: user.avatar_url,
            user_type: user.user_type.into(),
            wall_id: user.wall_id.id,
            is_blocked: user.is_blocked,
            password_hash: user.password_hash,
        }
    }
}

impl From<UserType> for schema::sea_orm_active_enums::UserType {
    fn from(user_type: UserType) -> Self {
        match user_type {
            UserType::Administrator => Self::Administrator,
            UserType::Moderator => Self::Moderator,
            UserType::Regular => Self::Regular,
        }
    }
}

impl From<schema::sea_orm_active_enums::UserType> for UserType {
    fn from(user_type: schema::sea_orm_active_enums::UserType) -> Self {
        match user_type {
            schema::sea_orm_active_enums::UserType::Administrator => Self::Administrator,
            schema::sea_orm_active_enums::UserType::Moderator => Self::Moderator,
            schema::sea_orm_active_enums::UserType::Regular => Self::Regular,
        }
    }
}
