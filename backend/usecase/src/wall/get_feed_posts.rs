use models::{
    domain::{post::Post, post_comment::PostComment, post_like::PostLike, user::User, Id},
    errors::AppResult,
};
use repository::{post_repository::PostRepository, wall_repository::WallRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetFeedPostsInput {
    pub user_id: Option<Id<User>>,
    pub pagination: (i64, i64),
}

pub type GetFeedPostsOutput = Vec<(Post, User, Vec<(PostComment, User)>, Vec<(PostLike, User)>)>;

pub struct GetFeedPostsUseCase<P: WallRepository> {
    wall_repository: P,
}

impl<P> GetFeedPostsUseCase<P>
where
    P: WallRepository,
{
    pub fn new(wall_repository: P) -> Self {
        Self { wall_repository }
    }

    pub async fn execute(&self, input: GetFeedPostsInput) -> AppResult<GetFeedPostsOutput> {
        let posts = self
            .wall_repository
            .get_feed(input.user_id, input.pagination.0, input.pagination.1)
            .await?;

        Ok(posts)
    }
}
