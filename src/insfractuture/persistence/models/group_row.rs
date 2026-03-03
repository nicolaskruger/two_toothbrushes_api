use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct GroupRow {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}
