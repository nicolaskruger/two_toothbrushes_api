use actix_web::web;

use crate::insfractuture::http::pix_controller::pix_ccontroller_factor;

pub mod config;
pub mod http;
pub mod persistence;
pub mod security;

pub fn controller_factory(conf: &mut web::ServiceConfig) {
    pix_ccontroller_factor(conf);
}
