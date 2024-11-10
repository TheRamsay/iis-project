pub mod group;
pub mod user;

// Write tests for use cases




#[cfg(test)]
fn lol() {
    use group::search_group::SearchGroupUseCase;
    use repository::group_repository::GroupRepository;

    let group_repository = GroupRepository { };
    let search_use_case = SearchGroupUseCase(group_repository);
}
