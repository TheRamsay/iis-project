use validator::{Validate, ValidationErrors};

use crate::schema;

use super::{user::User, wall::Wall, Id};

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct Group {
    pub id: Id<User>,
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    pub admin_id: Id<User>,
    pub wall_id: Id<Wall>,
}

impl Group {
    pub fn new(
        name: String,
        admin_id: Id<User>,
        wall_id: Id<Wall>,
    ) -> Result<Self, ValidationErrors> {
        let model = Self {
            id: Id::gen(),
            name,
            admin_id,
            wall_id,
        };

        model.validate()?;

        Ok(model)
    }
}

impl From<schema::group::Model> for Group {
    fn from(model: schema::group::Model) -> Self {
        Self {
            id: Id::new(model.id),
            name: model.name,
            admin_id: Id::new(model.admin_id),
            wall_id: Id::new(model.wall_id),
        }
    }
}

impl From<Group> for schema::group::Model {
    fn from(model: Group) -> Self {
        Self {
            id: model.id.id,
            name: model.name,
            admin_id: model.admin_id.id,
            wall_id: model.wall_id.id,
        }
    }
}
