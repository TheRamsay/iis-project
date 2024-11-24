use models::{
    domain::{
        post::Post,
        post_comment::PostComment,
        post_like::PostLike,
        user::User,
    },
    errors::AppResult,
};
use repository::wall_repository::WallRepository;
use uuid::Uuid;

use super::types::SortBy;

#[derive(Debug)]
pub struct GetWallPostsInput {
    pub id: Uuid,
    pub pagination: (i64, i64),
    pub sort_by: SortBy,
}

pub type GetWallPostsOutput = Vec<(Post, User, Vec<(PostComment, User)>, Vec<(PostLike, User)>)>;

pub struct GetWallPostsUseCase<P: WallRepository> {
    wall_repository: P,
}

impl<P> GetWallPostsUseCase<P>
where
    P: WallRepository,
{
    pub fn new(wall_repository: P) -> Self {
        Self { wall_repository }
    }

    pub async fn execute(&self, input: GetWallPostsInput) -> AppResult<GetWallPostsOutput> {
        let mut posts = self
            .wall_repository
            .get_wall_posts(input.id.into(), input.pagination.0, input.pagination.1)
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
