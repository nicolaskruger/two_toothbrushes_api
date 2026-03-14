use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Json},
};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    app::use_case::auth_group::{AuthGroupCase, AuthGroupInput},
    domain::{
        repository::{auth_repository::AuthRepository, group_repository::GroupRepository},
        services::password_hasher::PasswordHasher,
    },
    insfractuture::{
        config::settings,
        http::dto::{
            auth_error::AuthError, auth_request::AuthRequest, auth_response::AuthResponse,
        },
        jwt::jwt_auth_repository::JwtAuthRepository,
        persistence::postgresql_group_repository::PostgresqlGroupRepository,
        security::argon2_password_hasher::Aragon2PasswordHash,
    },
};

async fn _auth<R: GroupRepository, H: PasswordHasher, A: AuthRepository>(
    body: Json<AuthRequest>,
    mut auth_group: AuthGroupCase<R, H, A>,
) -> Result<AuthResponse, AuthError> {
    if body.validate().is_err() {
        Err(AuthError::WrongInput)
    } else {
        let input = AuthGroupInput {
            group: body.group.clone(),
            password: body.password.clone(),
        };

        match auth_group.execute(input).await {
            Ok(ok) => Ok(AuthResponse { token: ok.token }),
            Err(_) => Err(AuthError::WrongPassword),
        }
    }
}

#[post("/auth/")]
pub async fn auth(body: Json<AuthRequest>, pool: web::Data<PgPool>) -> impl Responder {
    let group_repository = PostgresqlGroupRepository::new(pool.get_ref().clone());

    let hasher = Aragon2PasswordHash;

    let settings = settings::Settings::load();

    let auth_repository = JwtAuthRepository::new(settings.auth_secret.to_string());

    let auth_group = AuthGroupCase::new(group_repository, hasher, auth_repository);

    match _auth(body, auth_group).await {
        Ok(ok) => HttpResponse::Ok().json(ok),
        Err(err) => match err {
            AuthError::WrongPassword => HttpResponse::Unauthorized().finish(),
            AuthError::WrongInput => HttpResponse::BadRequest().finish(),
        },
    }
}

pub fn auth_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(auth);
}
