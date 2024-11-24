use models::{
    domain::{post_comment::PostComment, Id},
    errors::AppResult,
};
use repository::post_comments_repository::PostCommentsRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommentPostInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub parent_id: Option<Uuid>,
}

pub struct CommentPostOutput {
    pub id: Uuid,
}

pub struct CommentPostUseCase<T>
where
    T: PostCommentsRepository,
{
    post_comments_repository: T,
}

impl<T> CommentPostUseCase<T>
where
    T: PostCommentsRepository,
{
    pub fn new(post_comments_repository: T) -> Self {
        Self {
            post_comments_repository,
        }
    }

    pub async fn execute(&self, input: CommentPostInput) -> AppResult<Option<CommentPostOutput>> {
        let post_comment = PostComment::new(
            Id::new(input.post_id),
            Id::new(input.user_id),
            input.content,
            input.parent_id.map(Id::new),
        )?;

        Ok(Some(CommentPostOutput {
            id: self.post_comments_repository.create(post_comment).await?.id,
        }))
    }
}
