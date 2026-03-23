use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Serialize, Clone)]
pub struct ConfirmUserRequest {
    pub id: Uuid,
    pub is_confirm: bool,
}
