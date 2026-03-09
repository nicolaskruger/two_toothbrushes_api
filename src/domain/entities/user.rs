use chrono::{DateTime, Utc};

use crate::domain::value_object::{group_id::GroupId, user_id::UserId};

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    group_id: GroupId,
    name: String,
    is_confirm: bool,
    created_at: DateTime<Utc>,
}

impl User {
    pub fn create(
        name: String,
        is_confirm: bool,
        group_id: GroupId,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: UserId::new(),
            name,
            is_confirm,
            group_id,
            created_at,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn group_id(&self) -> &GroupId {
        &self.group_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_confirm(&self) -> bool {
        self.is_confirm
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
