use actix_web::{HttpResponse, Responder, get, put, web};
use sqlx::PgPool;

use crate::{
    app::use_case::{
        confirm_user::{ConfirmUser, ConfirmUsersCase, ConfirmUsersInput},
        find_user_by_group_id::{FindUsersByGroupIdCase, FindUsersByGroupIdInput},
    },
    domain::value_object::group_id::GroupId,
    insfractuture::{
        http::dto::{
            confirm_user_request::ConfirmUserRequest, confirm_user_response::ConfirmUserResponse,
            user_info_response::UserInfoResponse,
        },
        persistence::postgresql_user_repository::PostgresqlUserRepository,
    },
};

#[get("/user/mine")]
pub async fn user_info(group_id: GroupId, pool: web::Data<PgPool>) -> impl Responder {
    let user_repository = PostgresqlUserRepository::new(pool.get_ref().clone());
    let mut find_user_by_id_user_case = FindUsersByGroupIdCase::new(user_repository);

    let result = find_user_by_id_user_case
        .execute(FindUsersByGroupIdInput { group_id })
        .await;

    match result {
        Ok(output) => {
            let users: Vec<_> = output
                .users
                .iter()
                .map(|u| UserInfoResponse {
                    id: u.id().as_uuid(),
                    name: u.name().to_string(),
                    is_confirm: u.is_confirm(),
                })
                .collect();

            HttpResponse::Ok().json(users)
        }
        Err(_) => HttpResponse::BadRequest().json("This user no longer exists"),
    }
}

#[put("/user/confirm")]
pub async fn confirm_user_rest(
    group_id: GroupId,
    users: web::Json<Vec<ConfirmUserRequest>>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user_repository = PostgresqlUserRepository::new(pool.get_ref().clone());
    let mut confirm_user_case = ConfirmUsersCase::new(user_repository);

    let users = users
        .clone()
        .into_iter()
        .map(|u| ConfirmUser {
            id: u.id,
            is_confirm: u.is_confirm,
        })
        .collect();

    let input = ConfirmUsersInput { group_id, users };

    let result = confirm_user_case.execute(input).await;

    match result {
        Ok(output) => {
            let users: Vec<_> = output
                .users
                .iter()
                .map(|u| ConfirmUserResponse {
                    id: u.id().as_uuid(),
                    name: u.name().to_string(),
                    is_confirm: u.is_confirm(),
                })
                .collect();

            HttpResponse::Ok().json(users)
        }
        Err(_) => HttpResponse::BadRequest().json("This user no longer exists"),
    }
}

pub fn user_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(user_info);
    conf.service(confirm_user_rest);
}
