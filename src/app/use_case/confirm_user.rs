use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{
    entities::user::User, repository::user_repository::UserRepository,
    value_object::group_id::GroupId,
};

pub struct ConfirmUser {
    pub id: Uuid,
    pub is_confirm: bool,
}

pub struct ConfirmUsersInput {
    pub group_id: GroupId,
    pub users: Vec<ConfirmUser>,
}

pub struct ConfirmUsersOutput {
    pub users: Vec<User>,
}

pub enum FindUserByGroupIdError {
    ThisGroupNotExists,
}

pub struct ConfirmUsersCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> ConfirmUsersCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &mut self,
        input: ConfirmUsersInput,
    ) -> Result<ConfirmUsersOutput, FindUserByGroupIdError> {
        let users = self
            .repository
            .find_by_group(&input.group_id)
            .await
            .map_err(|_| FindUserByGroupIdError::ThisGroupNotExists)?;

        let hash_map: HashMap<_, _> = input.users.iter().map(|u| (u.id, u)).collect();

        let users = users
            .iter()
            .filter(|u| hash_map.contains_key(&u.id().as_uuid()))
            .map(|u| {
                User::reconstitute(
                    u.id().clone(),
                    u.name().to_string(),
                    hash_map.get(&u.id().as_uuid()).unwrap().is_confirm,
                    u.group_id().clone(),
                    u.created_at(),
                )
            })
            .collect();

        let output = ConfirmUsersOutput { users };

        Ok(output)
    }
}
