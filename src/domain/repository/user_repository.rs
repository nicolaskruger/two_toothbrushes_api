use crate::domain::{entities::user::User, value_object::group_id::GroupId};

#[derive(Debug)]
pub enum UserRepositoryError {
    CouldNotCreate,
    SQLError,
}

pub trait UserRepository {
    fn create_user(&mut self, user: &User)
    -> impl Future<Output = Result<(), UserRepositoryError>>;
    fn count(&mut self) -> impl Future<Output = Result<i64, UserRepositoryError>>;
    fn find_by_group(
        &mut self,
        group_id: &GroupId,
    ) -> impl Future<Output = Result<Vec<User>, UserRepositoryError>>;
}
