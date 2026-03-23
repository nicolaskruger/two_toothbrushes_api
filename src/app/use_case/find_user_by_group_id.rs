use crate::domain::{
    entities::user::User, repository::user_repository::UserRepository,
    value_object::group_id::GroupId,
};

pub struct FindUserByGroupIdInput {
    pub group_id: GroupId,
}

pub struct FindUserByGroupIdOutput {
    pub users: Vec<User>,
}

pub enum FindUserByGroupIdError {
    ThisGroupNotExists,
}

pub struct FindUserByGroupIdCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> FindUserByGroupIdCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &mut self,
        input: FindUserByGroupIdInput,
    ) -> Result<FindUserByGroupIdOutput, FindUserByGroupIdError> {
        let users = self
            .repository
            .find_by_group(&input.group_id)
            .await
            .map_err(|_| FindUserByGroupIdError::ThisGroupNotExists)?;

        let output = FindUserByGroupIdOutput { users };

        Ok(output)
    }
}
