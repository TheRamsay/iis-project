use models::{domain::location::Location, errors::AppResult};
use repository::location_repository::LocationRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateLocationInput {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub picture_url: Option<String>,
}

pub struct CreateLocationOutput {
    pub id: Uuid,
}

pub struct CreateLocationUseCase<T>
where
    T: LocationRepository,
{
    Location_repository: T,
}

impl<T> CreateLocationUseCase<T>
where
    T: LocationRepository,
{
    pub fn new(Location_repository: T) -> Self {
        Self {
            Location_repository,
        }
    }

    pub async fn execute(&self, input: CreateLocationInput) -> AppResult<CreateLocationOutput> {
        let location = Location::new(
            input.picture_url,
            input.name,
            input.latitude,
            input.longitude,
        )?;

        Ok(CreateLocationOutput {
            id: self.Location_repository.create(location).await?.id,
        })
    }
}
