use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_group_visibility::PostGroupVisibility,
        post_tag::PostTag,
        post_user_visibility::PostUserVisibility,
        user::User,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};

use repository::post_visibility_repository::PostVisibilityRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct GetUserPostVisibilityInput {
    pub post_id: Uuid,
}

pub struct GetUserPostVisibilityOutput {
    pub visibilities: Vec<(PostUserVisibility, User)>,
}

pub struct GetUserPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    post_visibility_repository: T,
}

impl<T> GetUserPostVisibilityUseCase<T>
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
        input: GetUserPostVisibilityInput,
    ) -> AppResult<GetUserPostVisibilityOutput> {
        let result = self
            .post_visibility_repository
            .get_post_user_visibilities(Id::<Post>::new(input.post_id))
            .await?;

        Ok(GetUserPostVisibilityOutput {
            visibilities: result,
        })
    }
}
