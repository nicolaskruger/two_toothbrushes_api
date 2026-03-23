use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;

use crate::{
    app::use_case::find_user_by_group_id::{
        FindUsersByGroupIdCase, FindUsersByGroupIdInput, FindUsersByGroupIdOutput,
    },
    domain::value_object::group_id::GroupId,
    insfractuture::{
        http::dto::user_info_response::UserInfoResponse,
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

pub fn user_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(user_info);
}
