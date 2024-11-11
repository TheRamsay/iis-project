use validator::{Validate, ValidationErrors};

use crate::{errors::AppResult, schema};

use super::{post::Post, Id};

#[derive(Clone, Debug, PartialEq, Validate)]
pub struct Location {
    pub id: Id<Location>,
    #[validate(url)]
    pub picture_url: Option<String>,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn new(
        picture_url: Option<String>,
        name: String,
        latitude: f64,
        longitude: f64,
    ) -> Result<Self, ValidationErrors> {
        let location = Self {
            id: Id::gen(),
            picture_url,
            name,
            latitude,
            longitude,
        };

        location.validate()?;

        Ok(location)
    }
}

impl From<schema::location::Model> for Location {
    fn from(model: schema::location::Model) -> Self {
        Self {
            id: Id::new(model.id),
            picture_url: model.picture_url,
            name: model.name,
            latitude: model.latitude,
            longitude: model.longitude,
        }
    }
}

impl From<Location> for schema::location::Model {
    fn from(location: Location) -> Self {
        Self {
            id: location.id.id,
            picture_url: location.picture_url,
            name: location.name,
            latitude: location.latitude,
            longitude: location.longitude,
        }
    }
}
