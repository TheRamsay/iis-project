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
pub struct CreatePostTagInput {
    pub post_id: Uuid,
    pub tag: String,
}

pub struct CreatePostTagOutput {
    pub tag: String,
}

pub struct CreatePostTagUseCase<T>
where
    T: TagRepository,
{
    post_tag_repository: T,
}

impl<T> CreatePostTagUseCase<T>
where
    T: TagRepository,
{
    pub fn new(post_tag_repository: T) -> Self {
        Self {
            post_tag_repository,
        }
    }

    pub async fn execute(&self, input: CreatePostTagInput) -> AppResult<CreatePostTagOutput> {
        let tag = PostTag::new(Id::new(input.post_id), input.tag);

        Ok(CreatePostTagOutput {
            tag: self.post_tag_repository.create(tag).await?,
        })
    }
}
