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
pub struct CreateGroupPostVisibilityInput {
    pub post_id: Uuid,
    pub group_id: Uuid,
}

pub struct CreateGroupPostVisibilityOutput {
    pub post_id: Uuid,
    pub group_id: Uuid,
}

pub struct CreateGroupPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    post_visibility_repository: T,
}

impl<T> CreateGroupPostVisibilityUseCase<T>
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
        input: CreateGroupPostVisibilityInput,
    ) -> AppResult<CreateGroupPostVisibilityOutput> {
        let tag = PostGroupVisibility::new(Id::new(input.post_id), Id::new(input.group_id));
        let inserted = self
            .post_visibility_repository
            .create_group_visibility(tag)
            .await?;

        Ok(CreateGroupPostVisibilityOutput {
            post_id: inserted.0,
            group_id: inserted.1,
        })
    }
}
