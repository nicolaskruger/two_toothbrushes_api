#[derive(Clone, Debug)]
pub struct Pix {
    amount: f64,
    qr_code: String,
    qr_code_base64: String,
}
impl Pix {
    pub fn new(amount: f64, qr_code: String, qr_code_base64: String) -> Self {
        Self {
            amount,
            qr_code,
            qr_code_base64,
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
}
