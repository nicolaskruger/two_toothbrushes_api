use argon2::password_hash::PasswordHasher;
use argon2::{Argon2, PasswordVerifier};
use password_hash::{SaltString, rand_core::OsRng};

use crate::domain::{
    services::password_hasher::PasswordHasher as PasswordHasherTrait,
    value_object::hashed_password::HashedPassword,
};

pub struct Aragon2PasswordHash;

impl PasswordHasherTrait for Aragon2PasswordHash {
    fn hash(
        &self,
        password: &str,
    ) -> Result<crate::domain::value_object::hashed_password::HashedPassword, String> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        Ok(HashedPassword::new(password_hash))
    }

    fn verify(
        &self,
        password: &str,
        hash: crate::domain::value_object::hashed_password::HashedPassword,
    ) -> Result<bool, String> {
        let parsed_hash =
            password_hash::PasswordHash::new(hash.as_str()).map_err(|e| e.to_string())?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn aragon2_hash_test() {
        // cargo test aragon2_hash_test

        let cypher = Aragon2PasswordHash {};

        let hashed = cypher.hash("alogomora").expect("something on the way");

        assert_ne!(hashed.as_str(), "alogomora");
    }

    #[tokio::test]
    async fn aragon2_verify_test() {
        // cargo test aragon2_verify_test

        let cypher = Aragon2PasswordHash {};

        let hashed = cypher.hash("alogomora").expect("something on the way");
        let res = cypher.verify("alogomora", hashed).expect("error on verify");

        assert!(res);
    }
}
