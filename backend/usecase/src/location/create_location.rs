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
