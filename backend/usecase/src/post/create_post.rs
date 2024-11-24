use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_user_visibility::PostUserVisibility,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};
use repository::{
    post_repository::PostRepository, user_repository::UserRepository,
    wall_post_repository::WallPostRepository,
};
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

pub struct CreatePostUseCase<T, U, XD>
where
    T: PostRepository,
    U: WallPostRepository,
    XD: UserRepository,
{
    post_repository: T,
    wall_post_repository: U,
    user_repository: XD,
}

impl<T, U, XD> CreatePostUseCase<T, U, XD>
where
    T: PostRepository,
    U: WallPostRepository,
    XD: UserRepository,
{
    pub fn new(post_repository: T, wall_post_repository: U, user_repository: XD) -> Self {
        Self {
            post_repository,
            wall_post_repository,
            user_repository,
        }
    }

    pub async fn execute(&self, input: CreatePostInput) -> AppResult<CreatePostOutput> {
        let author = self
            .user_repository
            .get_by_id(Id::new(input.author_id))
            .await?
            .ok_or_else(|| AppError::NotFound("Author".to_string()))?;

        let post = Post::new(
            input.title,
            input.description,
            Id::new(input.author_id),
            input.post_type,
            input.content_url,
            input.visibility,
            input.location_id.map(Id::new),
        )?;

        let inserted_post_id = self.post_repository.create(post).await?.id;

        let wall_post = WallPost::new(inserted_post_id.into(), author.wall_id);

        self.wall_post_repository.create(wall_post).await?;

        Ok(CreatePostOutput {
            id: inserted_post_id,
        })
    }
}
