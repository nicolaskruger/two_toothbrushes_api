use actix_web::{HttpResponse, Responder, post, web};
use validator::Validate;

use crate::insfractuture::http::dto::create_payment_request::CreatePaymentRequest;

#[post("/pix/create/")]
async fn create(body: web::Json<CreatePaymentRequest>) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    HttpResponse::Ok().body(body.user_name.to_string())
}

pub fn pix_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create);
}
