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
use repository::{tag_repository::TagRepository, user_repository::UserRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetPostTagsInput {
    pub id: Uuid,
}

pub struct GetPostTagsOutput {
    pub tags: Vec<PostTag>,
}

pub struct GetPostTagsUseCase<T>
where
    T: TagRepository,
{
    post_tag_repository: T,
}

impl<T> GetPostTagsUseCase<T>
where
    T: TagRepository,
{
    pub fn new(post_tag_repository: T) -> Self {
        Self {
            post_tag_repository,
        }
    }

    pub async fn execute(&self, input: GetPostTagsInput) -> AppResult<GetPostTagsOutput> {
        let tags = self
            .post_tag_repository
            .get_tags_by_post_id(input.id.into())
            .await?
            .unwrap_or_else(Vec::new);

        Ok(GetPostTagsOutput { tags })
    }
}
