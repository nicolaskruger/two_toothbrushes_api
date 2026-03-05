use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub is_confirm: bool,
    pub created_at: DateTime<Utc>,
    pub group_id: Uuid,
}
