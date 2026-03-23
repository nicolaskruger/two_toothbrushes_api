use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserInfoResponse {
    pub id: Uuid,
    pub name: String,
    pub is_confirm: bool,
}
