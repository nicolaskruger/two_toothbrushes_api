#[derive(Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(hashed: String) -> Self {
        Self(hashed)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
