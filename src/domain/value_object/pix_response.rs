#[derive(Clone, Debug)]
pub struct PixResponse {
    amount: f64,
    qr_code: String,
    qr_code_base64: String,
    id: uuid::Uuid,
}
impl PixResponse {
    pub fn new(amount: f64, qr_code: String, qr_code_base64: String, id: uuid::Uuid) -> Self {
        Self {
            amount,
            qr_code,
            qr_code_base64,
            id,
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn qr_code(&self) -> String {
        self.qr_code.clone()
    }

    pub fn qr_code_base64(&self) -> String {
        self.qr_code_base64.clone()
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}
