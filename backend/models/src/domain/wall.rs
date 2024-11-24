use crate::schema;

use super::Id;

#[derive(Clone, Debug, PartialEq)]
pub struct Wall {
    pub id: Id<Wall>,
}

impl Wall {
    pub fn new() -> Self {
        Self { id: Id::gen() }
    }
}

impl From<schema::wall::Model> for Wall {
    fn from(model: schema::wall::Model) -> Self {
        Self {
            id: Id::new(model.id),
        }
    }
}

impl From<Wall> for schema::wall::Model {
    fn from(wall: Wall) -> Self {
        Self { id: wall.id.id }
    }
}
