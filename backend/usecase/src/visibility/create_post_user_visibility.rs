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
pub struct CreateUserPostVisibilityInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

pub struct CreateUserPostVisibilityOutput {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

pub struct CreateUserPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    post_visibility_repository: T,
}

impl<T> CreateUserPostVisibilityUseCase<T>
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
        input: CreateUserPostVisibilityInput,
    ) -> AppResult<CreateUserPostVisibilityOutput> {
        let tag = PostUserVisibility::new(Id::new(input.post_id), Id::new(input.user_id));
        let inserted = self
            .post_visibility_repository
            .create_user_visibility(tag)
            .await?;

        Ok(CreateUserPostVisibilityOutput {
            post_id: inserted.0,
            user_id: inserted.1,
        })
    }
}
