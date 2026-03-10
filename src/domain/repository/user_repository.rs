use crate::domain::entities::user::User;

#[derive(Debug)]
pub enum UserRepositoryError {
    CouldNotCreate,
    SQLError,
}

pub trait UserRepository {
    fn create_user(&mut self, user: &User)
    -> impl Future<Output = Result<(), UserRepositoryError>>;
    fn count(&mut self) -> impl Future<Output = Result<i64, UserRepositoryError>>;
}
