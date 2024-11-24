use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_tag::PostTag,
        post_user_visibility::PostUserVisibility,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};
use repository::{
    tag_repository::TagRepository, user_repository::UserRepository,
    wall_post_repository::WallPostRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct DeletePostTagInput {
    pub id: Uuid,
    pub tag: String,
}

pub struct DeletePostTagOutput {
    pub success: bool,
}

pub struct DeletePostTagUseCase<T>
where
    T: TagRepository,
{
    post_tag_repository: T,
}

impl<T> DeletePostTagUseCase<T>
where
    T: TagRepository,
{
    pub fn new(post_tag_repository: T) -> Self {
        Self {
            post_tag_repository,
        }
    }

    pub async fn execute(&self, input: DeletePostTagInput) -> AppResult<DeletePostTagOutput> {
        self.post_tag_repository
            .delete_by_id(Id::new(input.id), &input.tag)
            .await?;

        Ok(DeletePostTagOutput { success: true })
    }
}
