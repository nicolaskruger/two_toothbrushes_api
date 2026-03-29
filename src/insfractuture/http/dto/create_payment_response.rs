use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct CreatePaymentResponse {
    pub amount: f64,
    pub qr_code: String,
    pub qr_code_base64: String,
}
