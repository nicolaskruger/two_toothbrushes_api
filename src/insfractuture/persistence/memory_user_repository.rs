use std::sync::{Arc, Mutex};

use crate::domain::{
    entities::user::User,
    repository::user_repository::{UserRepository, UserRepositoryError},
};

pub struct MemoryUserRepository {
    pub users: Arc<Mutex<Vec<User>>>,
}

impl UserRepository for MemoryUserRepository {
    async fn create_user(
        &mut self,
        user: &crate::domain::entities::user::User,
    ) -> Result<(), UserRepositoryError> {
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
        Ok(())
    }

    async fn count(&mut self) -> Result<i64, UserRepositoryError> {
        let users = self.users.lock().unwrap();
        Ok(users.len() as i64)
    }

    async fn find_by_group(
        &mut self,
        _: &crate::domain::value_object::group_id::GroupId,
    ) -> Result<Vec<User>, UserRepositoryError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::domain::{entities::user::User, value_object::group_id::GroupId};

    use super::*;

    #[tokio::test]
    async fn memory_user_repository_create_test() {
        // cargo test memory_user_repository_create_test -- --ignored

        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let mut repo = MemoryUserRepository {
            users: arc_users.clone(),
        };

        let uuid = Uuid::parse_str("a0a4e7cc-aca4-4865-ae08-70d04cea1ed4").unwrap();
        let group_id = GroupId::from_uuid(uuid);

        let user = User::create("nicolas".to_string(), true, group_id, Utc::now());

        repo.create_user(&user).await.expect("error on insert");

        assert_eq!(repo.count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn memory_user_repository_count_test() {
        // cargo test memory_user_repository_create_test -- --ignored

        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let mut repo = MemoryUserRepository {
            users: arc_users.clone(),
        };

        let uuid = Uuid::parse_str("a0a4e7cc-aca4-4865-ae08-70d04cea1ed4").unwrap();
        let group_id = GroupId::from_uuid(uuid);

        let user = User::create("nicolas".to_string(), true, group_id, Utc::now());

        repo.create_user(&user).await.expect("error on insert");

        assert_eq!(repo.count().await.unwrap(), 1);
    }
}
