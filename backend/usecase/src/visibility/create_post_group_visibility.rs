use models::{
    domain::{
        group,
        post::{Post, PostType, PostVisibilityType},
        post_group_visibility::PostGroupVisibility,
        post_tag::PostTag,
        post_user_visibility::PostUserVisibility,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};

use repository::{
    group_repository::{self, GroupRepository},
    post_visibility_repository::PostVisibilityRepository,
    wall_post_repository::{self, WallPostRepository},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateGroupPostVisibilityInput {
    pub post_id: Uuid,
    pub group_id: Uuid,
}

pub struct CreateGroupPostVisibilityOutput {
    pub post_id: Uuid,
    pub group_id: Uuid,
}

pub struct CreateGroupPostVisibilityUseCase<T, W, G>
where
    T: PostVisibilityRepository,
    W: WallPostRepository,
    G: GroupRepository,
{
    post_visibility_repository: T,
    wall_post_repository: W,
    group_repository: G,
}

impl<T, W, G> CreateGroupPostVisibilityUseCase<T, W, G>
where
    T: PostVisibilityRepository,
    W: WallPostRepository,
    G: GroupRepository,
{
    pub fn new(
        post_visibility_repository: T,
        wall_post_repository: W,
        group_repository: G,
    ) -> Self {
        Self {
            post_visibility_repository,
            wall_post_repository,
            group_repository,
        }
    }

    pub async fn execute(
        &self,
        input: CreateGroupPostVisibilityInput,
    ) -> AppResult<CreateGroupPostVisibilityOutput> {
        let tag = PostGroupVisibility::new(Id::new(input.post_id), Id::new(input.group_id));
        let inserted = self
            .post_visibility_repository
            .create_group_visibility(tag)
            .await?;

        let group = self
            .group_repository
            .get_by_id(&Id::<group::Group>::new(input.group_id))
            .await?;

        let wall_post = if let Some((group, _user)) = group {
            WallPost::new(Id::new(input.post_id), group.wall_id)
        } else {
            return Err(AppError::NotFound("Group not found".into()));
        };
        self.wall_post_repository.create(wall_post).await?;

        Ok(CreateGroupPostVisibilityOutput {
            post_id: inserted.0,
            group_id: inserted.1,
        })
    }
}
