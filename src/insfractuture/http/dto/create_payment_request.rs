use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    #[validate(length(min = 1))]
    pub user_name: String,
    #[validate(length(min = 1))]
    pub message: String,
    #[validate(range(min = 1.))]
    pub reais: f64,
}
