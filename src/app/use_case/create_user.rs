use actix_web::Error;
use uuid::Uuid;

use crate::domain::{
    entities::user::User,
    repository::{group_repository::GroupRepository, user_repository::UserRepository},
    services::password_hasher::PasswordHasher,
    value_object::group_id::GroupId,
};

pub struct CreateUserInput {
    id_group: Uuid,
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct CreateUserOutput {
    id: Uuid,
}

pub struct CreateUserCase<RG, RU, H>
where
    RG: GroupRepository,
    RU: UserRepository,
    H: PasswordHasher,
{
    group_repossitor: RG,
    user_repossitor: RU,
    password_hasher: H,
}

#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    GroupNotFound,
}

impl<RG, RU, H> CreateUserCase<RG, RU, H>
where
    RG: GroupRepository,
    RU: UserRepository,
    H: PasswordHasher,
{
    pub fn new(group_repossitor: RG, user_repossitor: RU, password_hasher: H) -> Self {
        Self {
            group_repossitor,
            user_repossitor,
            password_hasher,
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

        todo!();
    }
}
pub async fn create_user(_: User) {
    todo!()
}

#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};

    use crate::{
        domain::entities::group::Group,
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

        let hasher = Aragon2PasswordHash {};

        let mut case = CreateUserCase::new(g_repo, u_repo, hasher);

        let res = case.execute(u_input).await;

        assert_eq!(res, Err(CreateUserError::GroupNotFound));

        let users = arc_users.lock().unwrap();

        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn create_group_test() {
        // cargo test create_group_test
    }
}
