use models::{
    domain::{
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
pub struct DeleteLocationInput {
    pub id: Uuid,
}

pub struct DeleteLocationOutput {
    pub success: bool,
}

pub struct DeleteLocationUseCase<T>
where
    T: LocationRepository,
{
    location_repository: T,
}

impl<T> DeleteLocationUseCase<T>
where
    T: LocationRepository,
{
    pub fn new(location_repository: T) -> Self {
        Self {
            location_repository,
        }
    }

    pub async fn execute(&self, input: DeleteLocationInput) -> AppResult<DeleteLocationOutput> {
        self.location_repository
            .delete_by_id(Id::new(input.id))
            .await?;

        Ok(DeleteLocationOutput { success: true })
    }
}
