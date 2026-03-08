use actix_web::Error;
use argon2::PasswordHasher;
use uuid::Uuid;

use crate::domain::{
    entities::user::User,
    repository::{group_repository::GroupRepository, user_repository::UserRepository},
};

pub struct CreateUserInput {
    id_group: Uuid,
    name: String,
}

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
    pub async fn execute(&mut self, input: CreateUserInput) -> Result<CreateUserOutput, Error> {
        todo!()
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
        insfractuture::security::argon2_password_hasher::Aragon2PasswordHash,
    };

    use super::*;

    struct TGroupRepository {
        pub groups: Arc<Mutex<Vec<Group>>>,
    }

    impl GroupRepository for TGroupRepository {
        async fn count(&mut self) -> Result<i64, actix_web::Error> {
            let groups = self.groups.lock().unwrap();
            Ok(groups.len() as i64)
        }

        async fn create(
            &mut self,
            group: &crate::domain::entities::group::Group,
        ) -> Result<(), actix_web::Error> {
            let mut groups = self.groups.lock().unwrap();
            groups.push(group.clone());
            Ok(())
        }

        async fn find_by_id(
            &mut self,
            id: &crate::domain::value_object::group_id::GroupId,
        ) -> Result<Group, Error> {
            todo!()
        }
    }

    #[tokio::test]
    async fn create_group_test() {
        // cargo test create_group_test
    }
}
