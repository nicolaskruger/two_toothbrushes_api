use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::domain::{
    entities::payment::Payment,
    repository::pix_repository::{PixError, PixRepository},
    value_object::pix::Pix,
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
struct PixResponse {
    pub point_of_interaction: PointOfInteraction,
}

impl PixRepository for MercadoPagoPixRepository {
    async fn register_payment(&mut self, _: Payment) -> Result<(), PixError> {
        todo!()
    }

    async fn create_pix(&mut self, amount: f64) -> Result<Pix, PixError> {
        if amount <= 0. {
            Err(PixError::LessOrEqualZero)
        } else {
            let client = reqwest::Client::new();

            let payment_json = json!({
                "transaction_amount": amount,
                "description": "Presente de casamento",
                "payment_method_id": "pix",
                "payer": {
                    "email": "guest@example.com"
                }
            });

            let key = Uuid::new_v4().to_string();

            let res = client
                .post("https://api.mercadopago.com/v1/payments")
                .json(&payment_json)
                .bearer_auth(self.payment_token.clone())
                .header("X-Idempotency-Key", key)
                .send()
                .await
                .map_err(|_| PixError::GatewayError)?;

            let res: PixResponse = res.json().await.map_err(|_| PixError::JsonParserError)?;
            let res = res.point_of_interaction.transaction_data;

            Ok(Pix::new(amount, res.qr_code, res.qr_code_base64))
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
        assert_eq!(err, PixError::LessOrEqualZero);

        let err = repo.create_pix(0.).await.unwrap_err();
        assert_eq!(err, PixError::LessOrEqualZero);
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
