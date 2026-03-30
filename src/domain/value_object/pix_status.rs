use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "pix_status", rename_all = "UPPERCASE")]
pub enum PixStatus {
    Pending,
    Paid,
}
