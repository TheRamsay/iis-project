use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_comment::PostComment,
        post_like::PostLike,
        post_visibility::PostVisibility,
        Id,
    },
    errors::AppResult,
};
use repository::{
    post_comments_repository::PostCommentsRepository, post_likes_repository::PostLikesRepository,
    post_repository::PostRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct UncommentPostInput {
    pub id: Uuid,
}

pub struct UncommentPostOutput {
    pub success: bool,
}

pub struct UncommentPostUseCase<T>
where
    T: PostCommentsRepository,
{
    post_comments_repository: T,
}

impl<T> UncommentPostUseCase<T>
where
    T: PostCommentsRepository,
{
    pub fn new(post_comments_repository: T) -> Self {
        Self {
            post_comments_repository,
        }
    }

    pub async fn execute(
        &self,
        input: UncommentPostInput,
    ) -> AppResult<Option<UncommentPostOutput>> {
        self.post_comments_repository
            .delete_by_id(Id::new(input.id))
            .await?;
        Ok(Some(UncommentPostOutput { success: true }))
    }
}
