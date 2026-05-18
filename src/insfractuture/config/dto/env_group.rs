use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnvGroup {
    pub name: String,
    pub code: String,
    pub password: String,
    pub users: Vec<String>,
}
