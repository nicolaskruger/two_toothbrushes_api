use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entities::user::User;

#[derive(Debug, Serialize)]
pub struct UserDetailResponse {
    pub id: Uuid,
    pub name: String,
    pub is_confirm: bool,
    pub group_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<&User> for UserDetailResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().as_uuid(),
            name: user.name().to_string(),
            is_confirm: user.is_confirm(),
            group_id: user.group_id().as_uuid(),
            created_at: user.created_at(),
        }
    }
}
