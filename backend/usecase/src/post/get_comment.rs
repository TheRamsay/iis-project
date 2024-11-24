use models::{
    domain::{
        post_comment::PostComment,
        Id,
    },
    errors::AppResult,
};
use repository::post_comments_repository::PostCommentsRepository;
use uuid::Uuid;

#[derive(Debug)]
pub struct GetCommentInput {
    pub id: Uuid,
}

#[derive(Clone)]
pub struct GetCommentOutput {
    pub comment: PostComment,
}

pub struct GetCommentUseCase<T>
where
    T: PostCommentsRepository,
{
    comment_repository: T,
}

impl<T> GetCommentUseCase<T>
where
    T: PostCommentsRepository,
{
    pub fn new(comment_repository: T) -> Self {
        Self { comment_repository }
    }

    pub async fn execute(&self, input: GetCommentInput) -> AppResult<Option<GetCommentOutput>> {
        match self
            .comment_repository
            .get_comment_by_id(Id::new(input.id))
            .await?
        {
            Some(comment) => Ok(Some(GetCommentOutput { comment })),
            None => Ok(None),
        }
    }
}
