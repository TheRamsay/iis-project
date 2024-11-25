use models::{
    domain::{post_comment::PostComment, wall_post::WallPost, Id},
    errors::AppResult,
};
use repository::{
    post_comments_repository::PostCommentsRepository, wall_post_repository::WallPostRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct AddPostToWallInput {
    pub post_id: Uuid,
    pub wall_id: Uuid,
}

pub struct AddPostToWallOutput {
    pub success: bool,
}

pub struct AddPostToWallUseCase<T>
where
    T: WallPostRepository,
{
    wall_post_repository: T,
}

impl<T> AddPostToWallUseCase<T>
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
        input: AddPostToWallInput,
    ) -> AppResult<Option<AddPostToWallOutput>> {
        let wall_post = WallPost::new(Id::new(input.post_id), Id::new(input.wall_id));

        self.wall_post_repository.create(wall_post).await?;

        Ok(Some(AddPostToWallOutput { success: true }))
    }
}
