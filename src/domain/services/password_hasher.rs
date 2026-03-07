use crate::domain::value_object::hashed_password::HashedPassword;

pub trait PasswordHasher {
    fn hash(&self, password: &str) -> Result<HashedPassword, String>;
    fn verify(&self, password: &str, hash: HashedPassword) -> Result<bool, String>;
}
