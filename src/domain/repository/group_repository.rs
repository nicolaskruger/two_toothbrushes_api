use crate::domain::{entities::group::Group, value_object::group_id::GroupId};

#[derive(Debug, PartialEq)]
pub enum GroupRepositoryError {
    CouldNotCreate,
    CouldNotUpdate,
    CouldNotDelete,
    NotFound,
    SQLError,
}

pub trait GroupRepository {
    fn count(&mut self) -> impl Future<Output = Result<i64, GroupRepositoryError>>;
    fn create(&mut self, group: &Group) -> impl Future<Output = Result<(), GroupRepositoryError>>;
    fn update(&mut self, group: &Group) -> impl Future<Output = Result<(), GroupRepositoryError>>;
    fn delete(&mut self, id: &GroupId) -> impl Future<Output = Result<(), GroupRepositoryError>>;
    fn find_by_name(
        &mut self,
        name: String,
    ) -> impl Future<Output = Result<Group, GroupRepositoryError>>;
    fn find_by_id(
        &mut self,
        id: &GroupId,
    ) -> impl Future<Output = Result<Group, GroupRepositoryError>>;
    fn find_all(&mut self) -> impl Future<Output = Result<Vec<Group>, GroupRepositoryError>>;
}
