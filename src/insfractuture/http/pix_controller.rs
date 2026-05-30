use actix_web::{HttpResponse, Responder, post, web};
use sqlx::PgPool;

use crate::{
    app::use_case::create_pix::{CreatePixCase, CreatePixError, CreatePixInput},
    domain::value_object::group_id::GroupId,
    insfractuture::{
        config::settings::Settings,
        http::dto::{
            create_payment_request::CreatePaymentRequest,
            create_payment_response::CreatePaymentResponse,
        },
        persistence::{
            mercado_pago_pix_repository::MercadoPagoPixRepository,
            postgresql_pix_repository::PostgresPixRepository,
        },
    },
};

#[post("/pix/create/")]
async fn create(
    body: web::Json<CreatePaymentRequest>,
    group_id: GroupId,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let input = CreatePixInput {
        amount: body.amount,
        group_id,
    };

    let settings = Settings::load();

    let pix_client_repository = MercadoPagoPixRepository::new(settings.payment_token);

    let pix_repository = PostgresPixRepository::new(pool.get_ref().clone());

    let mut case = CreatePixCase::new(pix_client_repository, pix_repository);

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
        Err(e) => match e {
                CreatePixError::InvalidAmount => HttpResponse::UnprocessableEntity()
                    .json("O valor do pagamento deve ser maior que zero"),
                CreatePixError::CouldNotCreate => HttpResponse::BadGateway()
                    .json("Falha na comunicação com o gateway de pagamento"),
                CreatePixError::CouldNotRegister => HttpResponse::InternalServerError()
                    .json("Falha ao registrar o pagamento"),
            },
    }
}

pub fn pix_ccontroller_factor(conf: &mut web::ServiceConfig) {
    conf.service(create);
}
