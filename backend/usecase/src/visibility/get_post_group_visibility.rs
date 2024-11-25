use models::{
    domain::{
        group::Group,
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
pub struct GetGroupPostVisibilityInput {
    pub post_id: Uuid,
}

pub struct GetGroupPostVisibilityOutput {
    pub visibilities: Vec<(PostGroupVisibility, Group)>,
}

pub struct GetGroupPostVisibilityUseCase<T>
where
    T: PostVisibilityRepository,
{
    post_visibility_repository: T,
}

impl<T> GetGroupPostVisibilityUseCase<T>
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
        input: GetGroupPostVisibilityInput,
    ) -> AppResult<GetGroupPostVisibilityOutput> {
        let result = self
            .post_visibility_repository
            .get_post_group_visibilities(Id::<Post>::new(input.post_id))
            .await?;

        Ok(GetGroupPostVisibilityOutput {
            visibilities: result,
        })
    }
}
