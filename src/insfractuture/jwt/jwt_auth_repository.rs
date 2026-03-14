use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::domain::{
    entities::{
        claim::{self, Claim},
        group::Group,
    },
    repository::auth_repository::{AuthRepository, AuthRepositoryError},
};

pub struct JwtAuthRepository {
    secret: String,
}

impl JwtAuthRepository {
    fn new(self, secret: String) -> Self {
        Self { secret }
    }
}

impl AuthRepository for JwtAuthRepository {
    async fn token(&mut self, group: &Group) -> Result<String, AuthRepositoryError> {
        let exp = Utc::now()
            .checked_add_signed(Duration::days(100000))
            .ok_or(AuthRepositoryError::GenTokenError)?
            .timestamp() as usize;

        let claim = Claim {
            group_id: group.id().as_uuid().to_string(),
            exp,
        };

        Ok(encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| AuthRepositoryError::GenTokenError)?)
    }
}
