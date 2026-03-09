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
        domain::{entities::group::Group, repository::group_repository::GroupRepositoryError},
        insfractuture::security::argon2_password_hasher::Aragon2PasswordHash,
    };

    use super::*;

    struct TGroupRepository {
        pub groups: Arc<Mutex<Vec<Group>>>,
    }

    impl GroupRepository for TGroupRepository {
        async fn count(&mut self) -> Result<i64, GroupRepositoryError> {
            let groups = self.groups.lock().unwrap();
            Ok(groups.len() as i64)
        }

        async fn create(
            &mut self,
            group: &crate::domain::entities::group::Group,
        ) -> Result<(), GroupRepositoryError> {
            let mut groups = self.groups.lock().unwrap();
            groups.push(group.clone());
            Ok(())
        }

        async fn find_by_id(
            &mut self,
            id: &crate::domain::value_object::group_id::GroupId,
        ) -> Result<Group, GroupRepositoryError> {
            let groups = self.groups.lock().unwrap();
            let group = groups
                .iter()
                .find(|g| g.id().as_uuid() == id.as_uuid())
                .ok_or(GroupRepositoryError::NotFound)?;

            Ok(group.clone())
        }
    }

    struct TUserRepository {
        pub users: Arc<Mutex<Vec<User>>>,
    }

    impl UserRepository for TUserRepository {
        async fn count(&mut self) -> Result<i64, actix_web::Error> {
            let groups = self.users.lock().unwrap();
            Ok(groups.len() as i64)
        }

        async fn create_user(&mut self, user: &User) -> Result<(), Error> {
            let mut groups = self.users.lock().unwrap();
            groups.push(user.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn do_not_create_user_when_no_group_test() {
        // cargo test do_not_create_user_when_no_group_test

        let g_uuid = Uuid::new_v4();

        let u_input = CreateUserInput {
            id_group: g_uuid,
            name: "u_name".to_string(),
        };

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let g_repo = TGroupRepository {
            groups: arc_groups.clone(),
        };

        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let u_repo = TUserRepository {
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
