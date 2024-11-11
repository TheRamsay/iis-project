use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::schema::{self, group_join_request, post_comment};

use super::{group::Group, post::Post, user::User, Id};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GroupJoinRequestStatus {
    Pending,
    Accepted,
    Rejected,
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
}

impl From<schema::group_join_request::Model> for GroupJoinRequest {
    fn from(model: schema::group_join_request::Model) -> Self {
        Self {
            id: Id::new(model.id),
            group_id: Id::new(model.group_id),
            user_id: Id::new(model.user_id),
            status: match model.status {
                schema::sea_orm_active_enums::GroupJoinStatusType::Pending => {
                    GroupJoinRequestStatus::Pending
                }
                schema::sea_orm_active_enums::GroupJoinStatusType::Accepted => {
                    GroupJoinRequestStatus::Accepted
                }
                schema::sea_orm_active_enums::GroupJoinStatusType::Rejected => {
                    GroupJoinRequestStatus::Rejected
                }
            },
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
            status: match model.status {
                GroupJoinRequestStatus::Pending => {
                    schema::sea_orm_active_enums::GroupJoinStatusType::Pending
                }
                GroupJoinRequestStatus::Accepted => {
                    schema::sea_orm_active_enums::GroupJoinStatusType::Accepted
                }
                GroupJoinRequestStatus::Rejected => {
                    schema::sea_orm_active_enums::GroupJoinStatusType::Rejected
                }
            },
            created_at: model.created_at.naive_local(),
            resolved_at: model.resolved_at.map(|date| date.naive_local()),
        }
    }
}
