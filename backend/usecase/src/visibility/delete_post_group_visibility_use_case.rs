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
pub struct DeleteGroupPostVisibilityInput {
    pub post_id: Uuid,
    pub group_id: Uuid,
}

pub struct DeleteGroupPostVisibilityOutput {
    pub success: bool,
}

pub struct DeleteGroupPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    post_visibility_repository: T,
}

impl<T> DeleteGroupPostVisibilityUseCase<T>
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
        input: DeleteGroupPostVisibilityInput,
    ) -> AppResult<DeleteGroupPostVisibilityOutput> {
        self.post_visibility_repository
            .delete_post_group_visibility(Id::new(input.post_id), Id::new(input.group_id))
            .await?;

        Ok(DeleteGroupPostVisibilityOutput { success: true })
    }
}
