use models::{
    domain::{
        post::Post,
        post_comment::PostComment,
        user::{User, UserType},
        Id,
    },
    errors::AppResult,
};
use repository::{
    post_comments_repository::PostCommentsRepository, post_likes_repository::PostLikesRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetPostCommentsInput {
    pub id: Uuid,
}

pub struct GetPostCommentsOutput {
    pub comments: Vec<PostComment>,
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
