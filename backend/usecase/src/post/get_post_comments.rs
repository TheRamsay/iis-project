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
pub struct GetPostCommentsInput {
    pub id: Uuid,
}

pub struct GetPostCommentsOutput {
    pub comments: Vec<(PostComment, User)>,
}

pub struct GetPostCommentsUseCase<T>
where
    T: PostCommentsRepository,
{
    post_repository: T,
}

impl<T> GetPostCommentsUseCase<T>
where
    T: PostCommentsRepository,
{
    pub fn new(post_repository: T) -> Self {
        Self { post_repository }
    }

    pub async fn execute(
        &self,
        input: GetPostCommentsInput,
    ) -> AppResult<Option<GetPostCommentsOutput>> {
        let result = self
            .post_repository
            .get_comments_by_post_id(Id::new(input.id))
            .await?;

        match result {
            Some(comments) => Ok(Some(GetPostCommentsOutput { comments: comments })),
            None => Ok(None),
        }
    }
}
