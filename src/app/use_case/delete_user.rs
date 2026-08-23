use uuid::Uuid;

use crate::domain::{
    repository::user_repository::{UserRepository, UserRepositoryError},
    value_object::user_id::UserId,
};

pub struct DeleteUserInput {
    pub id: Uuid,
}

#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    UserNotFound,
    CouldNotDelete,
}

pub struct DeleteUserCase<R>
where
    R: UserRepository,
{
    repository: R,
}

impl<R> DeleteUserCase<R>
where
    R: UserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&mut self, input: DeleteUserInput) -> Result<(), DeleteUserError> {
        self.repository
            .delete_user(&UserId::from_uuid(input.id))
            .await
            .map_err(|e| match e {
                UserRepositoryError::NotFound => DeleteUserError::UserNotFound,
                _ => DeleteUserError::CouldNotDelete,
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use chrono::Utc;

    use crate::{
        domain::{entities::user::User, value_object::group_id::GroupId},
        insfractuture::persistence::memory_user_repository::MemoryUserRepository,
    };

    use super::*;

    #[tokio::test]
    async fn delete_user_test() {
        // cargo test delete_user_test

        let user = User::create("name".to_string(), false, GroupId::new(), Utc::now());

        let arc_users = Arc::new(Mutex::new(vec![user.clone()]));

        let mut case = DeleteUserCase::new(MemoryUserRepository {
            users: arc_users.clone(),
        });

        case.execute(DeleteUserInput {
            id: user.id().as_uuid(),
        })
        .await
        .expect("user deleted");

        let users = arc_users.lock().unwrap();
        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn do_not_delete_unknown_user_test() {
        // cargo test do_not_delete_unknown_user_test

        let arc_users = Arc::new(Mutex::new(Vec::<User>::new()));

        let mut case = DeleteUserCase::new(MemoryUserRepository { users: arc_users });

        let res = case.execute(DeleteUserInput { id: Uuid::new_v4() }).await;

        assert_eq!(res.err(), Some(DeleteUserError::UserNotFound));
    }
}
