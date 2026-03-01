use actix_web::{App, HttpServer};
use dotenv::dotenv;
use two_toothbrushes_api::insfractuture::{config::migration::migrate, controller_factory};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("something whet wrong loading env variables");

    migrate()
        .await
        .expect("sometint went wrong with the migration");

    HttpServer::new(|| App::new().configure(controller_factory))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
