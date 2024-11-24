use models::{
    domain::{
        post::Post,
        post_comment::PostComment,
        post_like::PostLike,
        user::{User, UserType},
        wall,
    },
    errors::{AppError, AppResult},
};
use repository::{
    post_repository::PostRepository, user_repository::UserRepository,
    wall_repository::WallRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetWallPostsInput {
    pub id: Uuid,
    pub pagination: (i64, i64),
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
        let posts = self
            .wall_repository
            .get_wall_posts(input.id.into(), input.pagination.0, input.pagination.1)
            .await?;

        Ok(posts)
    }
}
