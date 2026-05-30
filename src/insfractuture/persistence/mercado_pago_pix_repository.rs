use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::domain::{
    repository::pix_client_repository::{PixClientError, PixClientRepository},
    value_object::pix_response::PixResponse,
};

pub struct MercadoPagoPixRepository {
    payment_token: String,
}

impl MercadoPagoPixRepository {
    pub fn new(payment_token: String) -> Self {
        Self { payment_token }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PixTransactionData {
    pub qr_code: String,
    pub qr_code_base64: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PointOfInteraction {
    pub transaction_data: PixTransactionData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct MercadoPagoPixResponse {
    pub point_of_interaction: PointOfInteraction,
}

impl PixClientRepository for MercadoPagoPixRepository {
    async fn create_pix(&mut self, amount: f64) -> Result<PixResponse, PixClientError> {
        if amount <= 0. {
            Err(PixClientError::LessOrEqualZero)
        } else {
            let client = reqwest::Client::new();

            let expiration = (Utc::now() + Duration::minutes(15))
                .format("%Y-%m-%dT%H:%M:%S%.3f-00:00")
                .to_string();

            let payment_json = json!({
                "transaction_amount": amount,
                "description": "Presente de casamento",
                "payment_method_id": "pix",
                "date_of_expiration": expiration,
                "payer": {
                    "email": "guest@example.com"
                }
            });

            let id = Uuid::new_v4();
            let key = id.to_string();

            let res = client
                .post("https://api.mercadopago.com/v1/payments")
                .json(&payment_json)
                .bearer_auth(self.payment_token.clone())
                .header("X-Idempotency-Key", key)
                .send()
                .await
                .map_err(|_| PixClientError::GatewayError)?;

            let res: MercadoPagoPixResponse = res
                .json()
                .await
                .map_err(|_| PixClientError::JsonParserError)?;
            let res = res.point_of_interaction.transaction_data;

            Ok(PixResponse::new(
                amount,
                res.qr_code,
                res.qr_code_base64,
                id,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::insfractuture::config::settings::Settings;

    use super::*;

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_pix_repository_do_not_zero_or_negative_amount() {
        // cargo test postgresql_pix_repository_do_not_zero_or_negative_amount -- --ignored
        dotenv().ok();
        let settings = Settings::load();

        let mut repo = MercadoPagoPixRepository::new(settings.payment_token);

        let err = repo.create_pix(-1.).await.unwrap_err();
        assert_eq!(err, PixClientError::LessOrEqualZero);

        let err = repo.create_pix(0.).await.unwrap_err();
        assert_eq!(err, PixClientError::LessOrEqualZero);
    }

    #[tokio::test]
    #[ignore = "database test"]
    async fn postgresql_pix_repository_create() {
        // cargo test postgresql_pix_repository_create -- --ignored --nocapture
        dotenv().ok();
        let settings = Settings::load();

        let mut repo = MercadoPagoPixRepository::new(settings.payment_token);

        let pix = repo.create_pix(1.).await.unwrap();

        assert_eq!(pix.clone().amount(), 1.);
        assert!(pix.clone().qr_code().len() > 5);
        assert!(pix.clone().qr_code_base64().len() > 5);
    }
}
