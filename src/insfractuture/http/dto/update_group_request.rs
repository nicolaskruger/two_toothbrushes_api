use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGroupRequest {
    #[validate(length(min = 1, message = "o nome não pode ser vazio"))]
    pub name: String,
    #[validate(length(min = 1, message = "o código não pode ser vazio"))]
    pub code: String,
    /// Omitir mantém a senha atual do grupo.
    #[validate(length(min = 1, message = "a senha não pode ser vazia"))]
    pub password: Option<String>,
}
