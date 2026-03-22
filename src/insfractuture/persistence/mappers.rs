use std::pin::Pin;

use actix_web::{Error, FromRequest};

use crate::domain::entities::group::Group;
use crate::domain::entities::user::User;
use crate::domain::repository::auth_repository::AuthRepository;
use crate::domain::value_object::group_id::GroupId;
use crate::domain::value_object::hashed_password::HashedPassword;
use crate::domain::value_object::user_id::UserId;
use crate::insfractuture::config::settings::Settings;
use crate::insfractuture::jwt::jwt_auth_repository::JwtAuthRepository;
use crate::insfractuture::persistence::models::group_row::GroupRow;
use crate::insfractuture::persistence::models::user_row::UserRow;

impl FromRequest for GroupId {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        fn t_err() -> Error {
            actix_web::error::ErrorUnauthorized("Invalid token")
        }

        async fn _from_request(req: actix_web::HttpRequest) -> Result<GroupId, Error> {
            let token = req.headers().get("Authorization");
            let token = token.ok_or(t_err())?;
            let token = token
                .to_str()
                .map_err(|_| t_err())?
                .strip_prefix("Bearer")
                .ok_or(t_err())?
                .trim()
                .to_string();

            let settings = Settings::load();
            let mut repo = JwtAuthRepository::new(settings.auth_secret);
            let group_id = repo.group_id(token).await.map_err(|_| t_err())?;

            Ok(group_id.clone())
        }

        Box::pin(async move { _from_request(req).await })
    }
}

impl From<&Group> for GroupRow {
    fn from(group: &Group) -> Self {
        Self {
            id: group.id().as_uuid(),
            name: group.name().to_string(),
            password: group.password().as_str().to_string(),
            created_at: group.created_at(),
        }
    }
}

impl From<GroupRow> for Group {
    fn from(group: GroupRow) -> Self {
        Group::reconstitute(
            GroupId::from_uuid(group.id),
            group.name.clone(),
            HashedPassword::new(group.password.clone()),
            group.created_at,
        )
    }
}

impl From<&UserRow> for User {
    fn from(user: &UserRow) -> Self {
        User::reconstitute(
            UserId::from_uuid(user.id),
            user.name.clone(),
            user.is_confirm,
            GroupId::from_uuid(user.group_id),
            user.created_at,
        )
    }
}

impl From<UserRow> for User {
    fn from(user: UserRow) -> Self {
        User::reconstitute(
            UserId::from_uuid(user.id),
            user.name.clone(),
            user.is_confirm,
            GroupId::from_uuid(user.group_id),
            user.created_at,
        )
    }
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().as_uuid(),
            name: user.name().to_string(),
            is_confirm: user.is_confirm(),
            created_at: user.created_at(),
            group_id: user.group_id().as_uuid(),
        }
    }
}
