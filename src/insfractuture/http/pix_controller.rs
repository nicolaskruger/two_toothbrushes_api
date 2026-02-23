use actix_web::{HttpResponse, Responder, post, web};

#[post("/pix/create/{reais}")]
async fn create(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(path.to_string())
}

pub fn pix_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create);
}
