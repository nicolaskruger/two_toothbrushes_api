use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AuthRequest {
    pub group: String,
    pub password: String,
}
