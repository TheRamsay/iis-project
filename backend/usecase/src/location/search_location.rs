use models::{
    domain::location::Location,
    errors::AppResult,
};
use repository::location_repository::LocationRepository;

#[derive(Debug)]
pub struct SearchLocationInput {
    pub query: String,
}

pub struct SearchLocationOutput {
    pub locations: Vec<Location>,
}

pub struct SearchLocationUseCase<T>
where
    T: LocationRepository,
{
    Location_repository: T,
}

impl<T> SearchLocationUseCase<T>
where
    T: LocationRepository,
{
    pub fn new(Location_repository: T) -> Self {
        Self {
            Location_repository,
        }
    }

    pub async fn execute(&self, input: SearchLocationInput) -> AppResult<SearchLocationOutput> {
        let locations = self
            .Location_repository
            .search(input.query)
            .await?
            .unwrap_or_else(Vec::new);

        Ok(SearchLocationOutput { locations })
    }
}
