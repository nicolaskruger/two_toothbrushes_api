use actix_web::Error;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    entities::group::Group,
    repository::{self, group_repository::GroupRepository},
    services::password_hasher::PasswordHasher,
};

pub struct CreateGroupInput {
    pub name: String,
    pub password: String,
}

pub struct CreateGroupOutput {
    pub id: Uuid,
}

pub struct CreateUserCase<R, H>
where
    R: GroupRepository,
    H: PasswordHasher,
{
    repository: R,
    hasher: H,
}

impl<R, H> CreateUserCase<R, H>
where
    R: GroupRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, hasher: H) -> Self {
        Self { repository, hasher }
    }

    pub async fn execute(&mut self, input: CreateGroupInput) -> Result<CreateGroupOutput, Error> {
        let password = self
            .hasher
            .hash(&input.password)
            .expect("can't hash password");

        let group = Group::create(input.name.clone(), password, Utc::now());

        self.repository
            .create(&group)
            .await
            .expect("can not insert group");

        let output = CreateGroupOutput {
            id: group.id().as_uuid(),
        };

        Ok(output)
    }
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
    }

    #[tokio::test]
    async fn create_user_test() {
        // cargo test create_user_test
        let input = CreateGroupInput {
            name: "name".to_string(),
            password: "pass".to_string(),
        };

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let repo = TGroupRepository {
            groups: arc_groups.clone(),
        };

        let hasher = Aragon2PasswordHash {};

        let mut case = CreateUserCase::new(repo, hasher);

        case.execute(input).await.expect("error");

        let groups = arc_groups.lock().unwrap();

        assert_eq!(groups.len(), 1);
    }
}
