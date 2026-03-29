use actix_web::{HttpResponse, Responder, post, web};
use validator::Validate;

use crate::{
    domain::value_object::group_id::GroupId,
    insfractuture::http::dto::create_payment_request::CreatePaymentRequest,
};

#[post("/pix/create/")]
async fn create(body: web::Json<CreatePaymentRequest>, group_id: GroupId) -> impl Responder {
    // if let Err(errors) = body.validate() {
    //     return HttpResponse::BadRequest().json(errors);
    // }
    //
    // HttpResponse::Ok().body(body.user_name.to_string())
    HttpResponse::Ok()
}

pub fn pix_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create);
}
