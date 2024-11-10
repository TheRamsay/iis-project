use models::{domain::{group::Group, user::User}, errors::AppResult};
use repository::group_repository::GroupRepository;

#[derive(Debug)]
pub struct SearchGroupInput {
    pub query: String,
}

pub struct SearchGroupOutput {
    pub groups: Vec<(Group, User)>,
}

pub struct SearchGroupUseCase<T>
where
    T: GroupRepository,
{
    group_repository: T,
}

impl<T> SearchGroupUseCase<T>
where
    T: GroupRepository,
{
    pub fn new(group_repository: T) -> Self {
        Self { group_repository }
    }

    pub async fn execute(&self, input: SearchGroupInput) -> AppResult<SearchGroupOutput> {
        let groups = self.group_repository.search(input.query).await?;

        Ok(SearchGroupOutput { groups })
    }
}
