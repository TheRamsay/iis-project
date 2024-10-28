use sea_orm::sqlx::types::chrono;
use validator::{Validate, ValidationErrors};

use crate::schema;

use super::{group::Group, user::User, Id};

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct GroupMember {
    pub user_id: Id<User>,
    pub group_id: Id<Group>,
    pub joined_at: chrono::NaiveDateTime,
}

impl GroupMember {
    pub fn new(user_id: Id<User>, group_id: Id<Group>) -> Result<Self, ValidationErrors> {
        let model = Self {
            user_id,
            group_id,
            joined_at: chrono::Utc::now().naive_utc(),
        };

        model.validate()?;

        Ok(model)
    }
}

impl From<schema::group_member::Model> for GroupMember {
    fn from(model: schema::group_member::Model) -> Self {
        Self {
            user_id: Id::new(model.user_id),
            group_id: Id::new(model.group_id),
            joined_at: model.joined_at,
        }
    }
}

impl From<GroupMember> for schema::group_member::Model {
    fn from(model: GroupMember) -> Self {
        Self {
            user_id: model.user_id.id,
            group_id: model.group_id.id,
            joined_at: model.joined_at,
        }
    }
}
