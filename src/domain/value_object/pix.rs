use chrono::{DateTime, Utc};

use crate::domain::value_object::{group_id::GroupId, pix_id::PixId, pix_status::PixStatus};

#[derive(Clone, Debug)]
pub struct Pix {
    amount: f64,
    qr_code: String,
    qr_code_base64: String,
    group_id: GroupId,
    id: PixId,
    status: PixStatus,
    created_at: DateTime<Utc>,
}

impl Pix {
    pub fn new(
        amount: f64,
        qr_code: String,
        qr_code_base64: String,
        group_id: GroupId,
        id: PixId,
        status: PixStatus,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            amount,
            qr_code,
            qr_code_base64,
            group_id,
            id,
            status,
            created_at,
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

    pub fn group_id(&self) -> GroupId {
        self.group_id.clone()
    }

    pub fn id(&self) -> PixId {
        self.id.clone()
    }

    pub fn status(&self) -> PixStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
