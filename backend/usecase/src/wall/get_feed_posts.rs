use models::{
    domain::{post::Post, post_comment::PostComment, post_like::PostLike, user::User, Id},
    errors::AppResult,
};
use repository::wall_repository::WallRepository;

use super::types::SortBy;

#[derive(Debug)]
pub struct GetFeedPostsInput {
    pub user_id: Option<Id<User>>,
    pub pagination: (i64, i64),
    pub sort_by: SortBy,
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
        let mut posts = self
            .wall_repository
            .get_feed(input.user_id, input.pagination.0, input.pagination.1)
            .await?;

        match input.sort_by {
            SortBy::Newest => {
                posts.sort_by(|a, b| b.0.created_at.cmp(&a.0.created_at));
            }
            SortBy::Oldest => {
                posts.sort_by(|a, b| a.0.created_at.cmp(&b.0.created_at));
            }
            SortBy::MostLiked => {
                posts.sort_by(|a, b| b.3.len().cmp(&a.3.len()));
            }
        }
        Ok(posts)
    }
}
