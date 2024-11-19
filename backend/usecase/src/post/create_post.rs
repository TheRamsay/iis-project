use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_visibility::PostVisibility,
        Id,
    },
    errors::AppResult,
};
use repository::post_repository::PostRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreatePostInput {
    pub title: String,
    pub description: String,
    pub author_id: Uuid,
    pub post_type: PostType,
    pub content_url: String,
    pub visibility: PostVisibilityType,
    pub location_id: Option<Uuid>,
}

pub struct CreatePostOutput {
    pub id: Uuid,
}

pub struct CreatePostUseCase<T>
where
    T: PostRepository,
{
    post_repository: T,
}

impl<T> CreatePostUseCase<T>
where
    T: PostRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, input: CreatePostInput) -> AppResult<CreatePostOutput> {
        let post = Post::new(
            input.title,
            input.description,
            Id::new(input.author_id),
            input.post_type,
            input.content_url,
            input.visibility,
            input.location_id.map(Id::new),
        )?;

        Ok(CreatePostOutput {
            id: self.post_repository.create(post).await?.id,
        })
    }
}
