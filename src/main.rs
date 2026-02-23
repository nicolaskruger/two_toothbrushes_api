use actix_web::{App, HttpServer};
use two_toothbrushes_api::insfractuture::controller_factory;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(controller_factory))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
