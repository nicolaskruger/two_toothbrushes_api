use crate::domain::entities::group::Group;

#[derive(Debug)]
pub enum AuthRepositoryError {
    GenTokenError,
}

pub trait AuthRepository {
    fn token(&mut self, group: &Group)
    -> impl Future<Output = Result<String, AuthRepositoryError>>;
}
