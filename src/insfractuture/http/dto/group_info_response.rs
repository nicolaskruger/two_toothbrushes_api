use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GroupInfoResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
}
