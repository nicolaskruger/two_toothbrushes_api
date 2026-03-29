use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    app::use_case::create_pix::{CreatePixCase, CreatePixInput},
    domain::value_object::group_id::GroupId,
    insfractuture::{
        config::settings::Settings,
        http::dto::{
            create_payment_request::CreatePaymentRequest,
            create_payment_response::CreatePaymentResponse,
        },
        persistence::mercado_pago_pix_repository::MercadoPagoPixRepository,
    },
};

#[post("/pix/create/")]
async fn create(body: web::Json<CreatePaymentRequest>, group_id: GroupId) -> impl Responder {
    let input = CreatePixInput {
        amount: body.amount,
        group_id,
    };

    let settings = Settings::load();

    let pix_repository = MercadoPagoPixRepository::new(settings.payment_token);

    let mut case = CreatePixCase::new(pix_repository);

    match case.execute(input).await {
        Ok(pix) => {
            let pix = pix.pix;
            let response = CreatePaymentResponse {
                amount: pix.amount(),
                qr_code: pix.qr_code(),
                qr_code_base64: pix.qr_code_base64(),
            };

            HttpResponse::Created().json(response)
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

pub fn pix_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create);
}
