use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::domain::entities::group::Group;

/// A senha do grupo nunca sai da API, nem em hash.
#[derive(Debug, Serialize)]
pub struct GroupDetailResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
}

impl From<&Group> for GroupDetailResponse {
    fn from(group: &Group) -> Self {
        Self {
            id: group.id().as_uuid(),
            name: group.name().to_string(),
            code: group.code().to_string(),
            created_at: group.created_at(),
        }
    }
}
