use crate::domain::{entities::user::User, repository::user_repository::UserRepository};

pub struct FindAllUsersOutput {
    pub users: Vec<User>,
}

#[derive(Debug, PartialEq)]
pub enum FindAllUsersError {
    CouldNotList,
}

pub struct FindAllUsersCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> FindAllUsersCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&mut self) -> Result<FindAllUsersOutput, FindAllUsersError> {
        let users = self
            .repository
            .find_all()
            .await
            .map_err(|_| FindAllUsersError::CouldNotList)?;

        Ok(FindAllUsersOutput { users })
    }
}
