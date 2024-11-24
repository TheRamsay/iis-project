use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::schema::{self, group_join_request, post_comment};

use super::{group::Group, post::Post, user::User, Id};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupJoinRequestStatus {
    Pending,
    Accepted,
    Rejected,
}

impl Default for GroupJoinRequestStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl Into<schema::sea_orm_active_enums::GroupJoinStatusType> for GroupJoinRequestStatus {
    fn into(self) -> schema::sea_orm_active_enums::GroupJoinStatusType {
        match self {
            GroupJoinRequestStatus::Pending => {
                schema::sea_orm_active_enums::GroupJoinStatusType::Pending
            }
            GroupJoinRequestStatus::Accepted => {
                schema::sea_orm_active_enums::GroupJoinStatusType::Accepted
            }
            GroupJoinRequestStatus::Rejected => {
                schema::sea_orm_active_enums::GroupJoinStatusType::Rejected
            }
        }
    }
}

impl From<schema::sea_orm_active_enums::GroupJoinStatusType> for GroupJoinRequestStatus {
    fn from(status: schema::sea_orm_active_enums::GroupJoinStatusType) -> Self {
        match status {
            schema::sea_orm_active_enums::GroupJoinStatusType::Pending => {
                GroupJoinRequestStatus::Pending
            }
            schema::sea_orm_active_enums::GroupJoinStatusType::Accepted => {
                GroupJoinRequestStatus::Accepted
            }
            schema::sea_orm_active_enums::GroupJoinStatusType::Rejected => {
                GroupJoinRequestStatus::Rejected
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct GroupJoinRequest {
    pub id: Id<GroupJoinRequest>,
    pub group_id: Id<Group>,
    pub user_id: Id<User>,
    pub status: GroupJoinRequestStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl GroupJoinRequest {
    pub fn new(group_id: Id<Group>, user_id: Id<User>) -> Self {
        Self {
            id: Id::gen(),
            group_id,
            user_id,
            created_at: Utc::now(),
            status: GroupJoinRequestStatus::Pending,
            resolved_at: None,
        }
    }

    pub fn accept(&mut self) {
        self.status = GroupJoinRequestStatus::Accepted;
        self.resolved_at = Some(Utc::now());
    }

    pub fn reject(&mut self) {
        self.status = GroupJoinRequestStatus::Rejected;
        self.resolved_at = Some(Utc::now());
    }
}

impl From<schema::group_join_request::Model> for GroupJoinRequest {
    fn from(model: schema::group_join_request::Model) -> Self {
        Self {
            id: Id::new(model.id),
            group_id: Id::new(model.group_id),
            user_id: Id::new(model.user_id),
            status: model.status.into(),
            created_at: model.created_at.and_utc(),
            resolved_at: model.resolved_at.map(|date| date.and_utc()),
        }
    }
}

impl From<GroupJoinRequest> for schema::group_join_request::Model {
    fn from(model: GroupJoinRequest) -> Self {
        Self {
            id: model.id.id,
            group_id: model.group_id.id,
            user_id: model.user_id.id,
            status: model.status.into(),
            created_at: model.created_at.naive_local(),
            resolved_at: model.resolved_at.map(|date| date.naive_local()),
        }
    }
}
