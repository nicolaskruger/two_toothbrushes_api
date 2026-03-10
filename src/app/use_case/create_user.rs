use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    entities::user::User,
    repository::{group_repository::GroupRepository, user_repository::UserRepository},
    value_object::group_id::GroupId,
};

pub struct CreateUserInput {
    pub id_group: Uuid,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct CreateUserOutput {
    pub id: Uuid,
}

pub struct CreateUserCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    group_repossitor: RG,
    user_repossitor: RU,
}

#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    GroupNotFound,
    CouldNotCreate,
}

impl<RG, RU> CreateUserCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    pub fn new(group_repossitor: RG, user_repossitor: RU) -> Self {
        Self {
            group_repossitor,
            user_repossitor,
        }
    }
    pub async fn execute(
        &mut self,
        input: CreateUserInput,
    ) -> Result<CreateUserOutput, CreateUserError> {
        let _ = self
            .group_repossitor
            .find_by_id(&GroupId::from_uuid(input.id_group))
            .await
            .map_err(|_| CreateUserError::GroupNotFound)?;

        let group_id = GroupId::from_uuid(input.id_group);

        let user = User::create(input.name.to_string(), false, group_id, Utc::now());

        self.user_repossitor
            .create_user(&user)
            .await
            .map_err(|_| CreateUserError::CouldNotCreate)?;

        Ok(CreateUserOutput {
            id: user.id().as_uuid(),
        })
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use chrono::Utc;

    use crate::{
        domain::{entities::group::Group, services::password_hasher::PasswordHasher},
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
    async fn do_not_create_user_when_no_group_test() {
        // cargo test do_not_create_user_when_no_group_test

        let g_uuid = Uuid::new_v4();

        let u_input = CreateUserInput {
            id_group: g_uuid,
            name: "u_name".to_string(),
        };

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let g_repo = MemoryGroupRepository {
            groups: arc_groups.clone(),
        };

        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let u_repo = MemoryUserRepository {
            users: arc_users.clone(),
        };

        let mut case = CreateUserCase::new(g_repo, u_repo);

        let res = case.execute(u_input).await;

        assert_eq!(res, Err(CreateUserError::GroupNotFound));

        let users = arc_users.lock().unwrap();

        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn create_group_test() {
        // cargo test create_group_test

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let mut g_repo = MemoryGroupRepository {
            groups: arc_groups.clone(),
        };

        let hasher = Aragon2PasswordHash {};

        let group_password = hasher.hash("passwod").unwrap();

        let group = Group::create("group".to_string(), group_password, Utc::now());

        g_repo.create(&group).await.expect("");

        {
            let groups = arc_groups.lock().unwrap();

            assert_eq!(groups.len(), 1);
        }

        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let u_repo = MemoryUserRepository {
            users: arc_users.clone(),
        };

        let u_input = CreateUserInput {
            id_group: group.id().as_uuid(),
            name: "u_name".to_string(),
        };

        let mut case = CreateUserCase::new(g_repo, u_repo);

        let res = case.execute(u_input).await;

        assert_ne!(res, Err(CreateUserError::GroupNotFound));

        let users = arc_users.lock().unwrap();

        assert_eq!(users.len(), 1);
    }
}
