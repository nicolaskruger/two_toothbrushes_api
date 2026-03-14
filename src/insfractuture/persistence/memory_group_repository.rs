use std::sync::{Arc, Mutex};

use crate::domain::repository::group_repository::GroupRepositoryError;

use crate::domain::{entities::group::Group, repository::group_repository::GroupRepository};

pub struct MemoryGroupRepository {
    pub groups: Arc<Mutex<Vec<Group>>>,
}

impl MemoryGroupRepository {
    pub fn new(groups: Arc<Mutex<Vec<Group>>>) -> Self {
        Self { groups }
    }
}

impl GroupRepository for MemoryGroupRepository {
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

    async fn find_by_name(&mut self, _: String) -> Result<Group, GroupRepositoryError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::{entities::group::Group, value_object::hashed_password::HashedPassword};

    use super::*;

    #[tokio::test]
    async fn memory_group_repository_count_test() {
        // cargo test memory_group_repository_count_test --

        let groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let mut repo = MemoryGroupRepository::new(groups);

        let count = repo.count().await.unwrap();

        assert_eq!(count, 0);
    }

    #[tokio::test]
    #[ignore = "database test"]
    async fn memory_group_repository_create_test() {
        // cargo test memory_group_repository_create_test --
        let groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let mut repo = MemoryGroupRepository::new(groups);

        let group = Group::create(
            "name".to_string(),
            HashedPassword::new("password".to_string()),
            Utc::now(),
        );

        repo.create(&group).await.expect("no error");

        let count = repo.count().await.unwrap();

        assert_eq!(count, 1);
    }

    #[tokio::test]
    #[ignore = "database test"]
    async fn memory_find_by_id_test() {
        // cargo test memory_find_by_id_test --  --nocapture
        let groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let mut repo = MemoryGroupRepository::new(groups);

        let group = Group::create(
            "name".to_string(),
            HashedPassword::new("password".to_string()),
            Utc::now(),
        );

        repo.create(&group).await.expect("no error");

        let _ = repo.find_by_id(group.id()).await.expect("fetched");
    }
}
