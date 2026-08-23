use std::sync::{Arc, Mutex};

use crate::domain::{
    entities::user::User,
    repository::user_repository::{UserRepository, UserRepositoryError},
};

#[derive(Clone)]
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
        group_id: &crate::domain::value_object::group_id::GroupId,
    ) -> Result<Vec<User>, UserRepositoryError> {
        let users = self.users.lock().unwrap();

        Ok(users
            .iter()
            .filter(|u| u.group_id().as_uuid() == group_id.as_uuid())
            .cloned()
            .collect())
    }

    async fn update_user(&mut self, user: &User) -> Result<(), UserRepositoryError> {
        let mut users = self.users.lock().unwrap();

        let slot = users
            .iter_mut()
            .find(|u| u.id().as_uuid() == user.id().as_uuid())
            .ok_or(UserRepositoryError::NotFound)?;

        *slot = user.clone();

        Ok(())
    }

    async fn delete_user(
        &mut self,
        id: &crate::domain::value_object::user_id::UserId,
    ) -> Result<(), UserRepositoryError> {
        let mut users = self.users.lock().unwrap();

        let before = users.len();
        users.retain(|u| u.id().as_uuid() != id.as_uuid());

        if users.len() == before {
            return Err(UserRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn find_by_id(
        &mut self,
        id: &crate::domain::value_object::user_id::UserId,
    ) -> Result<User, UserRepositoryError> {
        let users = self.users.lock().unwrap();

        users
            .iter()
            .find(|u| u.id().as_uuid() == id.as_uuid())
            .cloned()
            .ok_or(UserRepositoryError::NotFound)
    }

    async fn find_all(&mut self) -> Result<Vec<User>, UserRepositoryError> {
        let users = self.users.lock().unwrap();
        Ok(users.clone())
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
