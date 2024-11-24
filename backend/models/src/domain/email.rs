use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateArgs, ValidationError, ValidationErrors};

#[derive(Debug, Validate, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Self, ValidationErrors> {
        let model = Self { value };

        model.validate()?;

        Ok(model)
    }
}

impl TryFrom<String> for Email {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::new(value)?)
    }
}
