use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app::use_case::{
        create_user::{CreateUserCase, CreateUserError, CreateUserInput},
        delete_user::{DeleteUserCase, DeleteUserError, DeleteUserInput},
        find_all_users::FindAllUsersCase,
        find_user_by_id::{FindUserByIdCase, FindUserByIdInput},
        update_user::{UpdateUserCase, UpdateUserError, UpdateUserInput},
    },
    insfractuture::{
        http::{
            api_key::ApiKey,
            dto::{
                create_user_request::CreateUserRequest, update_user_request::UpdateUserRequest,
                user_detail_response::UserDetailResponse,
            },
        },
        persistence::{
            postgresql_group_repository::PostgresqlGroupRepository,
            postgresql_user_repository::PostgresqlUserRepository,
        },
    },
};

#[post("/admin/user")]
pub async fn create_user_rest(
    _: ApiKey,
    body: web::Json<CreateUserRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::UnprocessableEntity().json(e.to_string());
    }

    let mut create_user_case = CreateUserCase::new(
        PostgresqlGroupRepository::new(pool.get_ref().clone()),
        PostgresqlUserRepository::new(pool.get_ref().clone()),
    );

    let input = CreateUserInput {
        name: body.name.clone(),
        id_group: body.group_id,
    };

    let created = match create_user_case.execute(input).await {
        Ok(output) => output,
        Err(CreateUserError::GroupNotFound) => {
            return HttpResponse::NotFound().json("Grupo não encontrado");
        }
        Err(CreateUserError::CouldNotCreate) => {
            return HttpResponse::InternalServerError().json("Falha ao criar o usuário");
        }
    };

    let mut find_user_by_id_case =
        FindUserByIdCase::new(PostgresqlUserRepository::new(pool.get_ref().clone()));

    match find_user_by_id_case
        .execute(FindUserByIdInput { id: created.id })
        .await
    {
        Ok(output) => HttpResponse::Created().json(UserDetailResponse::from(&output.user)),
        Err(_) => HttpResponse::InternalServerError().json("Falha ao ler o usuário criado"),
    }
}

#[get("/admin/user")]
pub async fn list_users_rest(_: ApiKey, pool: web::Data<PgPool>) -> impl Responder {
    let mut find_all_users_case =
        FindAllUsersCase::new(PostgresqlUserRepository::new(pool.get_ref().clone()));

    match find_all_users_case.execute().await {
        Ok(output) => {
            let users: Vec<_> = output.users.iter().map(UserDetailResponse::from).collect();

            HttpResponse::Ok().json(users)
        }
        Err(_) => HttpResponse::InternalServerError().json("Falha ao listar os usuários"),
    }
}

#[get("/admin/user/{id}")]
pub async fn find_user_rest(
    _: ApiKey,
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut find_user_by_id_case =
        FindUserByIdCase::new(PostgresqlUserRepository::new(pool.get_ref().clone()));

    match find_user_by_id_case
        .execute(FindUserByIdInput {
            id: id.into_inner(),
        })
        .await
    {
        Ok(output) => HttpResponse::Ok().json(UserDetailResponse::from(&output.user)),
        Err(_) => HttpResponse::NotFound().json("Usuário não encontrado"),
    }
}

#[put("/admin/user/{id}")]
pub async fn update_user_rest(
    _: ApiKey,
    id: web::Path<Uuid>,
    body: web::Json<UpdateUserRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return HttpResponse::UnprocessableEntity().json(e.to_string());
    }

    let mut update_user_case = UpdateUserCase::new(
        PostgresqlGroupRepository::new(pool.get_ref().clone()),
        PostgresqlUserRepository::new(pool.get_ref().clone()),
    );

    let input = UpdateUserInput {
        id: id.into_inner(),
        name: body.name.clone(),
        is_confirm: body.is_confirm,
        group_id: body.group_id,
    };

    match update_user_case.execute(input).await {
        Ok(output) => HttpResponse::Ok().json(UserDetailResponse::from(&output.user)),
        Err(UpdateUserError::UserNotFound) => {
            HttpResponse::NotFound().json("Usuário não encontrado")
        }
        Err(UpdateUserError::GroupNotFound) => HttpResponse::NotFound().json("Grupo não encontrado"),
        Err(UpdateUserError::CouldNotUpdate) => {
            HttpResponse::InternalServerError().json("Falha ao atualizar o usuário")
        }
    }
}

#[delete("/admin/user/{id}")]
pub async fn delete_user_rest(
    _: ApiKey,
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let mut delete_user_case =
        DeleteUserCase::new(PostgresqlUserRepository::new(pool.get_ref().clone()));

    match delete_user_case
        .execute(DeleteUserInput {
            id: id.into_inner(),
        })
        .await
    {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(DeleteUserError::UserNotFound) => {
            HttpResponse::NotFound().json("Usuário não encontrado")
        }
        Err(DeleteUserError::CouldNotDelete) => {
            HttpResponse::InternalServerError().json("Falha ao remover o usuário")
        }
    }
}

pub fn user_crud_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create_user_rest);
    conf.service(list_users_rest);
    conf.service(find_user_rest);
    conf.service(update_user_rest);
    conf.service(delete_user_rest);
}
