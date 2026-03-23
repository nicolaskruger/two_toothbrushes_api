use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct ConfirmUserResponse {
    pub id: Uuid,
    pub name: String,
    pub is_confirm: bool,
}
