use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_group_visibility::PostGroupVisibility,
        post_tag::PostTag,
        post_user_visibility::PostUserVisibility,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};

use repository::post_visibility_repository::PostVisibilityRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct DeleteUserPostVisibilityInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

pub struct DeleteUserPostVisibilityOutput {
    pub success: bool,
}

pub struct DeleteUserPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    post_visibility_repository: T,
}

impl<T> DeleteUserPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    pub fn new(post_visibility_repository: T) -> Self {
        Self {
            post_visibility_repository,
        }
    }

    pub async fn execute(
        &self,
        input: DeleteUserPostVisibilityInput,
    ) -> AppResult<DeleteUserPostVisibilityOutput> {
        self.post_visibility_repository
            .delete_post_user_visibility(Id::new(input.post_id), Id::new(input.user_id))
            .await?;

        Ok(DeleteUserPostVisibilityOutput { success: true })
    }
}
