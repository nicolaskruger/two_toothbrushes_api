pub struct Payment {
    pub user_name: String,
    pub message: String,
    pub reais: f64,
    pub status: PaymentStatus,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Approved,
    Rejected,
}
