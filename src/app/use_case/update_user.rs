use uuid::Uuid;

use crate::domain::{
    entities::user::User,
    repository::{group_repository::GroupRepository, user_repository::UserRepository},
    value_object::{group_id::GroupId, user_id::UserId},
};

pub struct UpdateUserInput {
    pub id: Uuid,
    pub name: String,
    pub is_confirm: bool,
    pub group_id: Uuid,
}

pub struct UpdateUserOutput {
    pub user: User,
}

#[derive(Debug, PartialEq)]
pub enum UpdateUserError {
    UserNotFound,
    GroupNotFound,
    CouldNotUpdate,
}

pub struct UpdateUserCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    group_repository: RG,
    user_repository: RU,
}

impl<RG, RU> UpdateUserCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    pub fn new(group_repository: RG, user_repository: RU) -> Self {
        Self {
            group_repository,
            user_repository,
        }
    }

    pub async fn execute(
        &mut self,
        input: UpdateUserInput,
    ) -> Result<UpdateUserOutput, UpdateUserError> {
        let current = self
            .user_repository
            .find_by_id(&UserId::from_uuid(input.id))
            .await
            .map_err(|_| UpdateUserError::UserNotFound)?;

        let group_id = GroupId::from_uuid(input.group_id);

        self.group_repository
            .find_by_id(&group_id)
            .await
            .map_err(|_| UpdateUserError::GroupNotFound)?;

        let user = User::reconstitute(
            current.id().clone(),
            input.name,
            input.is_confirm,
            group_id,
            current.created_at(),
        );

        self.user_repository
            .update_user(&user)
            .await
            .map_err(|_| UpdateUserError::CouldNotUpdate)?;

        Ok(UpdateUserOutput { user })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::Utc;

    use crate::{
        domain::{
            entities::group::Group, services::password_hasher::PasswordHasher,
            value_object::hashed_password::HashedPassword,
        },
        insfractuture::{
            persistence::{
                memory_group_repository::MemoryGroupRepository,
                memory_user_repository::MemoryUserRepository,
            },
            security::argon2_password_hasher::Aragon2PasswordHash,
        },
    };

    use super::*;

    #[tokio::test]
    async fn do_not_update_user_when_group_is_unknown_test() {
        // cargo test do_not_update_user_when_group_is_unknown_test

        let group = Group::create(
            "group".to_string(),
            "code".to_string(),
            Aragon2PasswordHash.hash("password").unwrap(),
            Utc::now(),
        );

        let user = User::create("name".to_string(), false, group.id().clone(), Utc::now());

        let arc_groups = Arc::new(Mutex::new(vec![group]));
        let arc_users = Arc::new(Mutex::new(vec![user.clone()]));

        let mut case = UpdateUserCase::new(
            MemoryGroupRepository {
                groups: arc_groups.clone(),
            },
            MemoryUserRepository {
                users: arc_users.clone(),
            },
        );

        let res = case
            .execute(UpdateUserInput {
                id: user.id().as_uuid(),
                name: "new name".to_string(),
                is_confirm: true,
                group_id: Uuid::new_v4(),
            })
            .await;

        assert_eq!(res.err(), Some(UpdateUserError::GroupNotFound));

        let users = arc_users.lock().unwrap();
        assert_eq!(users[0].name(), "name");
    }

    #[tokio::test]
    async fn update_user_test() {
        // cargo test update_user_test

        let group = Group::create(
            "group".to_string(),
            "code".to_string(),
            HashedPassword::new("password".to_string()),
            Utc::now(),
        );

        let user = User::create("name".to_string(), false, group.id().clone(), Utc::now());

        let arc_groups = Arc::new(Mutex::new(vec![group.clone()]));
        let arc_users = Arc::new(Mutex::new(vec![user.clone()]));

        let mut case = UpdateUserCase::new(
            MemoryGroupRepository {
                groups: arc_groups.clone(),
            },
            MemoryUserRepository {
                users: arc_users.clone(),
            },
        );

        case.execute(UpdateUserInput {
            id: user.id().as_uuid(),
            name: "new name".to_string(),
            is_confirm: true,
            group_id: group.id().as_uuid(),
        })
        .await
        .expect("user updated");

        let users = arc_users.lock().unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name(), "new name");
        assert!(users[0].is_confirm());
    }

    #[tokio::test]
    async fn do_not_update_unknown_user_test() {
        // cargo test do_not_update_unknown_user_test

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));
        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let mut case = UpdateUserCase::new(
            MemoryGroupRepository {
                groups: arc_groups,
            },
            MemoryUserRepository { users: arc_users },
        );

        let res = case
            .execute(UpdateUserInput {
                id: Uuid::new_v4(),
                name: "new name".to_string(),
                is_confirm: true,
                group_id: Uuid::new_v4(),
            })
            .await;

        assert_eq!(res.err(), Some(UpdateUserError::UserNotFound));
    }
}
