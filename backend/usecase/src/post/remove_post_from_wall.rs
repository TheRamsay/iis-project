use models::{
    domain::{post_comment::PostComment, wall_post::WallPost, Id},
    errors::AppResult,
};
use repository::{
    post_comments_repository::PostCommentsRepository, wall_post_repository::WallPostRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct RemovePostToWallInput {
    pub post_id: Uuid,
    pub wall_id: Uuid,
}

pub struct RemovePostToWallOutput {
    pub success: bool,
}

pub struct RemovePostToWallUseCase<T>
where
    T: WallPostRepository,
{
    wall_post_repository: T,
}

impl<T> RemovePostToWallUseCase<T>
where
    T: WallPostRepository,
{
    pub fn new(wall_post_repository: T) -> Self {
        Self {
            wall_post_repository,
        }
    }

    pub async fn execute(
        &self,
        input: RemovePostToWallInput,
    ) -> AppResult<Option<RemovePostToWallOutput>> {
        let wall_post = WallPost::new(Id::new(input.post_id), Id::new(input.wall_id));

        self.wall_post_repository.delete(wall_post).await?;

        Ok(Some(RemovePostToWallOutput { success: true }))
    }
}
