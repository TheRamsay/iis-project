use models::{
    domain::{
        location::Location,
        post::{Post, PostType, PostVisibilityType},
        post_user_visibility::PostUserVisibility,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};
use repository::{
    location_repository::LocationRepository, post_repository::PostRepository,
    user_repository::UserRepository, wall_post_repository::WallPostRepository,
};
use uuid::Uuid;

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
