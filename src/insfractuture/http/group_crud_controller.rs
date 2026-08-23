use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::use_case::{
        create_group::{CreateGroupCase, CreateGroupError, CreateGroupInput},
        delete_group::{DeleteGroupCase, DeleteGroupError, DeleteGroupInput},
        find_all_groups::FindAllGroupsCase,
        find_group_by_id::{FindGroupByIdCase, FindGroupByIdInput},
        update_group::{UpdateGroupCase, UpdateGroupError, UpdateGroupInput},
    },
    domain::value_object::group_id::GroupId,
    insfractuture::{
        http::{
            api_key::ApiKey,
            dto::{
                create_group_request::CreateGroupRequest,
                group_detail_response::GroupDetailResponse,
                update_group_request::UpdateGroupRequest,
            },
        },
        persistence::{
            postgresql_group_repository::PostgresqlGroupRepository,
            postgresql_user_repository::PostgresqlUserRepository,
        },
        security::argon2_password_hasher::Aragon2PasswordHash,
    },
};

#[post("/admin/group")]
pub async fn create_group_rest(
    _: ApiKey,
    body: web::Json<CreateGroupRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::UnprocessableEntity().json(e.to_string());
    }

    let mut create_group_case = CreateGroupCase::new(
        PostgresqlGroupRepository::new(pool.get_ref().clone()),
        Aragon2PasswordHash,
    );

    let input = CreateGroupInput {
        name: body.name.clone(),
        code: body.code.clone(),
        password: body.password.clone(),
    };

    match create_group_case.execute(input).await {
        Ok(output) => HttpResponse::Created().json(GroupDetailResponse::from(&output.group)),
        Err(CreateGroupError::DuplicatedCode) => {
            HttpResponse::Conflict().json("Já existe um grupo com este código")
        }
        Err(CreateGroupError::CouldNotHashPassword) => {
            HttpResponse::InternalServerError().json("Falha ao proteger a senha do grupo")
        }
        Err(CreateGroupError::CouldNotCreate) => {
            HttpResponse::InternalServerError().json("Falha ao criar o grupo")
        }
    }
}

#[get("/admin/group")]
pub async fn list_groups_rest(_: ApiKey, pool: web::Data<PgPool>) -> impl Responder {
    let mut find_all_groups_case =
        FindAllGroupsCase::new(PostgresqlGroupRepository::new(pool.get_ref().clone()));

    match find_all_groups_case.execute().await {
        Ok(output) => {
            let groups: Vec<_> = output
                .groups
                .iter()
                .map(GroupDetailResponse::from)
                .collect();

            HttpResponse::Ok().json(groups)
        }
        Err(_) => HttpResponse::InternalServerError().json("Falha ao listar os grupos"),
    }
}

#[get("/admin/group/{id}")]
pub async fn find_group_rest(
    _: ApiKey,
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut find_group_by_id_case =
        FindGroupByIdCase::new(PostgresqlGroupRepository::new(pool.get_ref().clone()));

    match find_group_by_id_case
        .execute(FindGroupByIdInput {
            group_id: GroupId::from_uuid(id.into_inner()),
        })
        .await
    {
        Ok(output) => HttpResponse::Ok().json(GroupDetailResponse::from(&output.group)),
        Err(_) => HttpResponse::NotFound().json("Grupo não encontrado"),
    }
}

#[put("/admin/group/{id}")]
pub async fn update_group_rest(
    _: ApiKey,
    id: web::Path<Uuid>,
    body: web::Json<UpdateGroupRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::UnprocessableEntity().json(e.to_string());
    }

    let mut update_group_case = UpdateGroupCase::new(
        PostgresqlGroupRepository::new(pool.get_ref().clone()),
        Aragon2PasswordHash,
    );

    let input = UpdateGroupInput {
        id: id.into_inner(),
        name: body.name.clone(),
        code: body.code.clone(),
        password: body.password.clone(),
    };

    match update_group_case.execute(input).await {
        Ok(output) => HttpResponse::Ok().json(GroupDetailResponse::from(&output.group)),
        Err(UpdateGroupError::GroupNotFound) => {
            HttpResponse::NotFound().json("Grupo não encontrado")
        }
        Err(UpdateGroupError::DuplicatedCode) => {
            HttpResponse::Conflict().json("Já existe um grupo com este código")
        }
        Err(UpdateGroupError::CouldNotHashPassword) => {
            HttpResponse::InternalServerError().json("Falha ao proteger a senha do grupo")
        }
        Err(UpdateGroupError::CouldNotUpdate) => {
            HttpResponse::InternalServerError().json("Falha ao atualizar o grupo")
        }
    }
}

#[delete("/admin/group/{id}")]
pub async fn delete_group_rest(
    _: ApiKey,
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut delete_group_case = DeleteGroupCase::new(
        PostgresqlGroupRepository::new(pool.get_ref().clone()),
        PostgresqlUserRepository::new(pool.get_ref().clone()),
    );

    match delete_group_case
        .execute(DeleteGroupInput {
            id: id.into_inner(),
        })
        .await
    {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(DeleteGroupError::GroupNotFound) => {
            HttpResponse::NotFound().json("Grupo não encontrado")
        }
        Err(DeleteGroupError::GroupHasUsers) => {
            HttpResponse::Conflict().json("O grupo ainda possui usuários vinculados")
        }
        Err(DeleteGroupError::CouldNotDelete) => {
            HttpResponse::InternalServerError().json("Falha ao remover o grupo")
        }
    }
}

pub fn group_crud_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create_group_rest);
    conf.service(list_groups_rest);
    conf.service(find_group_rest);
    conf.service(update_group_rest);
    conf.service(delete_group_rest);
}
