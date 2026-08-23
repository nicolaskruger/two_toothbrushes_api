use uuid::Uuid;

use crate::domain::{
    repository::{
        group_repository::{GroupRepository, GroupRepositoryError},
        user_repository::UserRepository,
    },
    value_object::group_id::GroupId,
};

pub struct DeleteGroupInput {
    pub id: Uuid,
}

#[derive(Debug, PartialEq)]
pub enum DeleteGroupError {
    GroupNotFound,
    GroupHasUsers,
    CouldNotDelete,
}

pub struct DeleteGroupCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    group_repository: RG,
    user_repository: RU,
}

impl<RG, RU> DeleteGroupCase<RG, RU>
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

    pub async fn execute(&mut self, input: DeleteGroupInput) -> Result<(), DeleteGroupError> {
        let group_id = GroupId::from_uuid(input.id);

        self.group_repository
            .find_by_id(&group_id)
            .await
            .map_err(|_| DeleteGroupError::GroupNotFound)?;

        // users.group_id has no ON DELETE CASCADE, so an occupied group must be
        // emptied first instead of blowing up on a foreign key violation.
        let users = self
            .user_repository
            .find_by_group(&group_id)
            .await
            .map_err(|_| DeleteGroupError::CouldNotDelete)?;

        if !users.is_empty() {
            return Err(DeleteGroupError::GroupHasUsers);
        }

        self.group_repository
            .delete(&group_id)
            .await
            .map_err(|e| match e {
                GroupRepositoryError::NotFound => DeleteGroupError::GroupNotFound,
                _ => DeleteGroupError::CouldNotDelete,
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::Utc;

    use crate::{
        domain::{
            entities::{group::Group, user::User},
            value_object::hashed_password::HashedPassword,
        },
        insfractuture::persistence::{
            memory_group_repository::MemoryGroupRepository,
            memory_user_repository::MemoryUserRepository,
        },
    };

    use super::*;

    fn a_group() -> Group {
        Group::create(
            "group".to_string(),
            "code".to_string(),
            HashedPassword::new("password".to_string()),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn delete_empty_group_test() {
        // cargo test delete_empty_group_test

        let group = a_group();

        let arc_groups = Arc::new(Mutex::new(vec![group.clone()]));
        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let mut case = DeleteGroupCase::new(
            MemoryGroupRepository {
                groups: arc_groups.clone(),
            },
            MemoryUserRepository { users: arc_users },
        );

        case.execute(DeleteGroupInput {
            id: group.id().as_uuid(),
        })
        .await
        .expect("group deleted");

        let groups = arc_groups.lock().unwrap();
        assert_eq!(groups.len(), 0);
    }

    #[tokio::test]
    async fn do_not_delete_group_with_users_test() {
        // cargo test do_not_delete_group_with_users_test

        let group = a_group();

        let user = User::create("name".to_string(), false, group.id().clone(), Utc::now());

        let arc_groups = Arc::new(Mutex::new(vec![group.clone()]));
        let arc_users = Arc::new(Mutex::new(vec![user]));

        let mut case = DeleteGroupCase::new(
            MemoryGroupRepository {
                groups: arc_groups.clone(),
            },
            MemoryUserRepository { users: arc_users },
        );

        let res = case
            .execute(DeleteGroupInput {
                id: group.id().as_uuid(),
            })
            .await;

        assert_eq!(res.err(), Some(DeleteGroupError::GroupHasUsers));

        let groups = arc_groups.lock().unwrap();
        assert_eq!(groups.len(), 1);
    }

    #[tokio::test]
    async fn do_not_delete_unknown_group_test() {
        // cargo test do_not_delete_unknown_group_test

        let arc_groups = Arc::new(Mutex::new(Vec::<Group>::new()));
        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let mut case = DeleteGroupCase::new(
            MemoryGroupRepository {
                groups: arc_groups,
            },
            MemoryUserRepository { users: arc_users },
        );

        let res = case.execute(DeleteGroupInput { id: Uuid::new_v4() }).await;

        assert_eq!(res.err(), Some(DeleteGroupError::GroupNotFound));
    }
}
