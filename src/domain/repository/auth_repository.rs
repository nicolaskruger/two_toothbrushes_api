use crate::domain::{entities::group::Group, value_object::group_id::GroupId};

#[derive(Debug)]
pub enum AuthRepositoryError {
    GenTokenError,
}

pub trait AuthRepository {
    fn token(&mut self, group: &Group)
    -> impl Future<Output = Result<String, AuthRepositoryError>>;

    fn group_id(
        &mut self,
        token: String,
    ) -> impl Future<Output = Result<GroupId, AuthRepositoryError>>;
}
