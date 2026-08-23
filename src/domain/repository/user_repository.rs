use crate::domain::{
    entities::user::User,
    value_object::{group_id::GroupId, user_id::UserId},
};

#[derive(Debug, PartialEq)]
pub enum UserRepositoryError {
    CouldNotCreate,
    CouldNotUpdate,
    CouldNotDelete,
    NotFound,
    SQLError,
}

pub trait UserRepository {
    fn create_user(&mut self, user: &User)
    -> impl Future<Output = Result<(), UserRepositoryError>>;
    fn update_user(&mut self, user: &User)
    -> impl Future<Output = Result<(), UserRepositoryError>>;
    fn delete_user(&mut self, id: &UserId) -> impl Future<Output = Result<(), UserRepositoryError>>;
    fn count(&mut self) -> impl Future<Output = Result<i64, UserRepositoryError>>;
    fn find_by_id(
        &mut self,
        id: &UserId,
    ) -> impl Future<Output = Result<User, UserRepositoryError>>;
    fn find_by_group(
        &mut self,
        group_id: &GroupId,
    ) -> impl Future<Output = Result<Vec<User>, UserRepositoryError>>;
    fn find_all(&mut self) -> impl Future<Output = Result<Vec<User>, UserRepositoryError>>;
}
