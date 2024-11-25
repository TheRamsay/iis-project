use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_tag::PostTag,
        post_user_visibility::PostUserVisibility,
        wall_post::WallPost,
        Id,
    },
    errors::{AppError, AppResult},
};
use repository::{tag_repository::TagRepository, user_repository::UserRepository};

#[derive(Debug)]
pub struct SearchPostTagInput {
    pub query: String,
}

pub struct SearchPostTagOutput {
    pub tags: Vec<PostTag>,
}

pub struct SearchPostTagUseCase<T>
where
    T: TagRepository,
{
    post_tag_repository: T,
}

impl<T> SearchPostTagUseCase<T>
where
    T: TagRepository,
{
    pub fn new(post_tag_repository: T) -> Self {
        Self {
            post_tag_repository,
        }
    }

    pub async fn execute(&self, input: SearchPostTagInput) -> AppResult<SearchPostTagOutput> {
        let tags = self
            .post_tag_repository
            .search(input.query)
            .await?
            .unwrap_or_else(Vec::new);

        let mut grouped_tags = std::collections::HashMap::new();
        for tag in tags {
            grouped_tags
                .entry(tag.tag.clone())
                .or_insert_with(Vec::new)
                .push(tag);
        }

        let grouped_tags_vec = grouped_tags
            .into_iter()
            .flat_map(|(_, tags)| tags)
            .collect();

        Ok(SearchPostTagOutput {
            tags: grouped_tags_vec,
        })
    }
}
