use serde::Serialize;

#[derive(Serialize)]
pub struct GroupInfoResponse {
    pub name: String,
}
