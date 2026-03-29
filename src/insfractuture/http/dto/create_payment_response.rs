use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentResponse {
    pub amount: f64,
    pub qr_code: String,
    pub qr_code_base64: String,
}
