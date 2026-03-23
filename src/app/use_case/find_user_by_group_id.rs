use crate::domain::{
    entities::user::User, repository::user_repository::UserRepository,
    value_object::group_id::GroupId,
};

pub struct FindUsersByGroupIdInput {
    pub group_id: GroupId,
}

pub struct FindUsersByGroupIdOutput {
    pub users: Vec<User>,
}

pub enum FindUserByGroupIdError {
    ThisGroupNotExists,
}

pub struct FindUsersByGroupIdCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> FindUsersByGroupIdCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &mut self,
        input: FindUsersByGroupIdInput,
    ) -> Result<FindUsersByGroupIdOutput, FindUserByGroupIdError> {
        let users = self
            .repository
            .find_by_group(&input.group_id)
            .await
            .map_err(|_| FindUserByGroupIdError::ThisGroupNotExists)?;

        let output = FindUsersByGroupIdOutput { users };

        Ok(output)
    }
}
