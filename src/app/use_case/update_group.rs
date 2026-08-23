use uuid::Uuid;

use crate::domain::{
    entities::group::Group,
    repository::group_repository::{GroupRepository, GroupRepositoryError},
    services::password_hasher::PasswordHasher,
    value_object::group_id::GroupId,
};

pub struct UpdateGroupInput {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    /// `None` keeps the password the group already has.
    pub password: Option<String>,
}

pub struct UpdateGroupOutput {
    pub group: Group,
}

#[derive(Debug, PartialEq)]
pub enum UpdateGroupError {
    GroupNotFound,
    DuplicatedCode,
    CouldNotHashPassword,
    CouldNotUpdate,
}

pub struct UpdateGroupCase<R, H>
where
    R: GroupRepository,
    H: PasswordHasher,
{
    repository: R,
    hasher: H,
}

impl<R, H> UpdateGroupCase<R, H>
where
    R: GroupRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, hasher: H) -> Self {
        Self { repository, hasher }
    }

    pub async fn execute(
        &mut self,
        input: UpdateGroupInput,
    ) -> Result<UpdateGroupOutput, UpdateGroupError> {
        let current = self
            .repository
            .find_by_id(&GroupId::from_uuid(input.id))
            .await
            .map_err(|_| UpdateGroupError::GroupNotFound)?;

        let password = match input.password {
            Some(password) => self
                .hasher
                .hash(&password)
                .map_err(|_| UpdateGroupError::CouldNotHashPassword)?,
            None => current.password().clone(),
        };

        let group = Group::reconstitute(
            current.id().clone(),
            input.name,
            input.code,
            password,
            current.created_at(),
        );

        self.repository.update(&group).await.map_err(|e| match e {
            GroupRepositoryError::NotFound => UpdateGroupError::GroupNotFound,
            GroupRepositoryError::CouldNotCreate => UpdateGroupError::DuplicatedCode,
            _ => UpdateGroupError::CouldNotUpdate,
        })?;

        Ok(UpdateGroupOutput { group })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::Utc;

    use crate::{
        domain::value_object::hashed_password::HashedPassword,
        insfractuture::{
            persistence::memory_group_repository::MemoryGroupRepository,
            security::argon2_password_hasher::Aragon2PasswordHash,
        },
    };

    use super::*;

    #[tokio::test]
    async fn update_group_keeps_password_when_not_informed_test() {
        // cargo test update_group_keeps_password_when_not_informed_test

        let group = Group::create(
            "group".to_string(),
            "code".to_string(),
            HashedPassword::new("kept".to_string()),
            Utc::now(),
        );

        let arc_groups = Arc::new(Mutex::new(vec![group.clone()]));

        let mut case = UpdateGroupCase::new(
            MemoryGroupRepository {
                groups: arc_groups.clone(),
            },
            Aragon2PasswordHash,
        );

        case.execute(UpdateGroupInput {
            id: group.id().as_uuid(),
            name: "new name".to_string(),
            code: "new code".to_string(),
            password: None,
        })
        .await
        .expect("group updated");

        let groups = arc_groups.lock().unwrap();
        assert_eq!(groups[0].name(), "new name");
        assert_eq!(groups[0].code(), "new code");
        assert_eq!(groups[0].password().as_str(), "kept");
    }

    #[tokio::test]
    async fn update_group_hashes_new_password_test() {
        // cargo test update_group_hashes_new_password_test

        let group = Group::create(
            "group".to_string(),
            "code".to_string(),
            HashedPassword::new("kept".to_string()),
            Utc::now(),
        );

        let arc_groups = Arc::new(Mutex::new(vec![group.clone()]));

        let mut case = UpdateGroupCase::new(
            MemoryGroupRepository {
                groups: arc_groups.clone(),
            },
            Aragon2PasswordHash,
        );

        case.execute(UpdateGroupInput {
            id: group.id().as_uuid(),
            name: "group".to_string(),
            code: "code".to_string(),
            password: Some("secret".to_string()),
        })
        .await
        .expect("group updated");

        let groups = arc_groups.lock().unwrap();
        let stored = groups[0].password().as_str();

        assert_ne!(stored, "kept");
        assert_ne!(stored, "secret");
        assert!(
            Aragon2PasswordHash
                .verify("secret", HashedPassword::new(stored.to_string()))
                .expect("hash is readable")
        );
    }

    #[tokio::test]
    async fn do_not_update_unknown_group_test() {
        // cargo test do_not_update_unknown_group_test

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));

        let mut case = UpdateGroupCase::new(
            MemoryGroupRepository {
                groups: arc_groups,
            },
            Aragon2PasswordHash,
        );

        let res = case
            .execute(UpdateGroupInput {
                id: Uuid::new_v4(),
                name: "name".to_string(),
                code: "code".to_string(),
                password: None,
            })
            .await;

        assert_eq!(res.err(), Some(UpdateGroupError::GroupNotFound));
    }
}
