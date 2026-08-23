use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "o nome não pode ser vazio"))]
    pub name: String,
    pub group_id: Uuid,
}
