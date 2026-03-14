use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use two_toothbrushes_api::insfractuture::{
    config::{migration::migrate, settings::Settings},
    controller_factory,
    init::init_container,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("something whet wrong loading env variables");

    migrate()
        .await
        .expect("sometint went wrong with the migration");

    let settings = Settings::load();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&settings.postgresql_url)
        .await
        .expect("not connected");

    init_container(pool.clone()).await;

    HttpServer::new(|| App::new().configure(controller_factory))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
