use chrono::{DateTime, Utc};

use crate::domain::value_object::{group_id::GroupId, hashed_password::HashedPassword};

pub struct Group {
    id: GroupId,
    name: String,
    password: HashedPassword,
    created_at: DateTime<Utc>,
}

impl Group {
    pub fn reconstitute(
        id: GroupId,
        name: String,
        password: HashedPassword,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            password,
            created_at,
        }
    }
    pub fn create(name: String, password: HashedPassword, created_at: DateTime<Utc>) -> Self {
        Self {
            id: GroupId::new(),
            name,
            password,
            created_at,
        }
    }

    pub fn id(&self) -> &GroupId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn password(&self) -> &HashedPassword {
        &self.password
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
