use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;

use crate::{
    app::use_case::find_group_by_id::{FindGroupByIdCase, FindGroupByIdInput},
    domain::value_object::group_id::GroupId,
    insfractuture::{
        http::dto::group_info_response::GroupInfoResponse,
        persistence::postgresql_group_repository::PostgresqlGroupRepository,
    },
};

#[get("/group/me")]
pub async fn group_info(group_id: GroupId, pool: web::Data<PgPool>) -> impl Responder {
    let group_repository = PostgresqlGroupRepository::new(pool.get_ref().clone());
    let mut find_group_by_id_user_case = FindGroupByIdCase::new(group_repository);

    let result = find_group_by_id_user_case
        .execute(FindGroupByIdInput { group_id })
        .await;

    match result {
        Ok(output) => {
            let response = GroupInfoResponse {
                id: output.group.id().as_uuid(),
                name: output.group.name().to_string(),
                code: output.group.code().to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::BadRequest().json("This user no longer exists"),
    }
}

pub fn group_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(group_info);
}
