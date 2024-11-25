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

use repository::{
    group_repository::GroupRepository, post_visibility_repository::PostVisibilityRepository,
    wall_post_repository::WallPostRepository,
};
use uuid::Uuid;

use crate::group;

#[derive(Debug)]
pub struct DeleteGroupPostVisibilityInput {
    pub post_id: Uuid,
    pub group_id: Uuid,
}

pub struct DeleteGroupPostVisibilityOutput {
    pub success: bool,
}

pub struct DeleteGroupPostVisibilityUseCase<T, W, G>
where
    T: PostVisibilityRepository,
    W: WallPostRepository,
    G: GroupRepository,
{
    post_visibility_repository: T,
    wall_post_repository: W,
    group_repository: G,
}

impl<T, W, G> DeleteGroupPostVisibilityUseCase<T, W, G>
where
    T: PostVisibilityRepository,
    W: WallPostRepository,
    G: GroupRepository,
{
    pub fn new(
        post_visibility_repository: T,
        wall_post_repository: W,
        group_repository: G,
    ) -> Self {
        Self {
            post_visibility_repository,
            wall_post_repository,
            group_repository,
        }
    }

    pub async fn execute(
        &self,
        input: DeleteGroupPostVisibilityInput,
    ) -> AppResult<DeleteGroupPostVisibilityOutput> {
        self.post_visibility_repository
            .delete_post_group_visibility(Id::new(input.post_id), Id::new(input.group_id))
            .await?;

        let group_id = Id::new(input.group_id);
        let group = self.group_repository.get_by_id(&group_id).await?;

        let wall_post = if let Some((group, _user)) = group {
            WallPost::new(Id::new(input.post_id), group.wall_id)
        } else {
            return Err(AppError::NotFound("Group not found".into()));
        };
        self.wall_post_repository.delete(wall_post).await?;

        Ok(DeleteGroupPostVisibilityOutput { success: true })
    }
}
