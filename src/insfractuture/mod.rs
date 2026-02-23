use actix_web::web;

use crate::insfractuture::http::pix_controller::pix_ccontroller_factor;

pub mod http;

pub fn controller_factory(conf: &mut web::ServiceConfig) {
    pix_ccontroller_factor(conf);
}
