use models::{
    domain::{post::Post, post_comment::PostComment, post_like::PostLike, user::User, Id},
    errors::AppResult,
};
use repository::{
    post_repository::PostRepository, wall_post_repository::WallPostRepository,
    wall_repository::WallRepository,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct GetTagPostsInput {
    pub tag: String,
    pub user_id: Option<Id<User>>,
    pub pagination: (i64, i64),
}

pub type GetTagPostsOutput = Vec<(Post, User, Vec<(PostComment, User)>, Vec<(PostLike, User)>)>;

pub struct GetTagPostsUseCase<P: WallRepository> {
    wall_repository: P,
}

impl<P> GetTagPostsUseCase<P>
where
    P: WallRepository,
{
    pub fn new(wall_repository: P) -> Self {
        Self { wall_repository }
    }

    pub async fn execute(&self, input: GetTagPostsInput) -> AppResult<GetTagPostsOutput> {
        let posts = self
            .wall_repository
            .get_posts_by_tag(
                input.tag,
                input.user_id,
                input.pagination.0,
                input.pagination.1,
            )
            .await?;

        Ok(posts)
    }
}
