use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::domain::{
    entities::{claim::Claim, group::Group},
    repository::auth_repository::{AuthRepository, AuthRepositoryError},
    value_object::group_id::GroupId,
};

pub struct JwtAuthRepository {
    secret: String,
}

impl JwtAuthRepository {
    pub fn new(secret: String) -> Self {
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

        let out = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|_| AuthRepositoryError::GenTokenError)?;

        Ok(out)
    }

    async fn group_id(
        &mut self,
        token: String,
    ) -> Result<crate::domain::value_object::group_id::GroupId, AuthRepositoryError> {
        let claim = decode::<Claim>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthRepositoryError::GenTokenError)?;

        let uuid = Uuid::parse_str(&claim.claims.group_id)
            .map_err(|_| AuthRepositoryError::GenTokenError)?;

        Ok(GroupId::from_uuid(uuid))
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::{entities::group::Group, value_object::hashed_password::HashedPassword};

    use super::*;
    #[tokio::test]
    async fn decode_encode_test() {
        // cargo test decode_encode_test

        let mut repo = JwtAuthRepository::new("secret".into());

        let group = Group::create(
            "family".into(),
            "code".into(),
            HashedPassword::new("password".into()),
            Utc::now(),
        );

        let token = repo.token(&group).await.unwrap();

        let group_id = repo.group_id(token).await.unwrap();

        assert_eq!(group_id.as_uuid(), group.id().as_uuid());
    }
}
