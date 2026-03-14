use sqlx::{Pool, Postgres};

use crate::{
    app::use_case::{create_group::CreateGroupCase, create_user::CreateUserCase},
    insfractuture::{
        config::settings::Settings,
        init::init_controller::init_group,
        persistence::{
            postgresql_group_repository::PostgresqlGroupRepository,
            postgresql_user_repository::PostgresqlUserRepository,
        },
        security::argon2_password_hasher::Aragon2PasswordHash,
    },
};

pub mod init_controller;

pub async fn init_container(pool: Pool<Postgres>) {
    let settings = Settings::load();
    let hasher = Aragon2PasswordHash;
    let create_group_case =
        CreateGroupCase::new(PostgresqlGroupRepository::new(pool.clone()), hasher);
    let create_user_case = CreateUserCase::new(
        PostgresqlGroupRepository::new(pool.clone()),
        PostgresqlUserRepository::new(pool.clone()),
    );

    init_group(
        &settings,
        create_group_case,
        create_user_case,
        PostgresqlGroupRepository::new(pool.clone()),
        PostgresqlUserRepository::new(pool.clone()),
    )
    .await;
}
