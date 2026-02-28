use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct PaymentRow {
    pub id: Uuid,
    pub message: String,
    pub reais: f64,
    pub status: PaymentStatusRow,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatusRow {
    Pending,
    Approved,
    Rejected,
}
