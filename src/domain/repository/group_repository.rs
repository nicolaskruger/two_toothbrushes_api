use crate::domain::{entities::group::Group, value_object::group_id::GroupId};

#[derive(Debug)]
pub enum GroupRepositoryError {
    CouldNotCreate,
    NotFound,
    SQLError,
}

pub trait GroupRepository {
    fn count(&mut self) -> impl Future<Output = Result<i64, GroupRepositoryError>>;
    fn create(&mut self, group: &Group) -> impl Future<Output = Result<(), GroupRepositoryError>>;
    fn find_by_id(
        &mut self,
        id: &GroupId,
    ) -> impl Future<Output = Result<Group, GroupRepositoryError>>;
}
