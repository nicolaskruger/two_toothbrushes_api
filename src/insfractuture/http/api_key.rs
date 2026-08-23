use std::pin::Pin;

use actix_web::{Error, FromRequest};

use crate::insfractuture::config::settings::Settings;

pub const API_KEY_HEADER: &str = "x-api-key";

/// Guard extractor: every handler that takes an `ApiKey` argument only runs when
/// the request carries the `x-api-key` header matching `API_KEY` from the env.
pub struct ApiKey;

impl FromRequest for ApiKey {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();

        fn k_err() -> Error {
            actix_web::error::ErrorUnauthorized("Invalid api key")
        }

        async fn _from_request(req: actix_web::HttpRequest) -> Result<ApiKey, Error> {
            let key = req.headers().get(API_KEY_HEADER).ok_or(k_err())?;
            let key = key.to_str().map_err(|_| k_err())?.trim();

            let settings = Settings::load();

            if constant_time_eq(key.as_bytes(), settings.api_key.as_bytes()) {
                Ok(ApiKey)
            } else {
                Err(k_err())
            }
        }

        Box::pin(async move { _from_request(req).await })
    }
}

/// Compares without short-circuiting so a wrong key never leaks how much of it matched.
fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    left.iter()
        .zip(right.iter())
        .fold(0u8, |acc, (l, r)| acc | (l ^ r))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_time_eq_test() {
        // cargo test constant_time_eq_test
        assert!(constant_time_eq(b"secret", b"secret"));
        assert!(!constant_time_eq(b"secret", b"secreT"));
        assert!(!constant_time_eq(b"secret", b"secret "));
        assert!(!constant_time_eq(b"", b"secret"));
    }
}
