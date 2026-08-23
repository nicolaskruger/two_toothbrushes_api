use uuid::Uuid;

use crate::domain::{
    entities::user::User, repository::user_repository::UserRepository,
    value_object::user_id::UserId,
};

pub struct FindUserByIdInput {
    pub id: Uuid,
}

pub struct FindUserByIdOutput {
    pub user: User,
}

#[derive(Debug, PartialEq)]
pub enum FindUserByIdError {
    UserNotFound,
}

pub struct FindUserByIdCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> FindUserByIdCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &mut self,
        input: FindUserByIdInput,
    ) -> Result<FindUserByIdOutput, FindUserByIdError> {
        let user = self
            .repository
            .find_by_id(&UserId::from_uuid(input.id))
            .await
            .map_err(|_| FindUserByIdError::UserNotFound)?;

        Ok(FindUserByIdOutput { user })
    }
}
