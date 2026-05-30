use chrono::Utc;

use crate::domain::{
    repository::{
        pix_client_repository::{PixClientError, PixClientRepository},
        pix_repository::PixRepository,
    },
    value_object::{
        group_id::GroupId, pix::Pix, pix_id::PixId, pix_response::PixResponse,
        pix_status::PixStatus,
    },
};

pub struct CreatePixInput {
    pub group_id: GroupId,
    pub amount: f64,
}

pub struct CreatePixOutput {
    pub pix: PixResponse,
}

pub struct CreatePixCase<PC, PR>
where
    PC: PixClientRepository,
    PR: PixRepository,
{
    pix_client_repository: PC,
    pix_repository: PR,
}

#[derive(Debug, PartialEq)]
pub enum CreatePixError {
    InvalidAmount,
    CouldNotCreate,
    CouldNotRegister,
}

impl<PC, PR> CreatePixCase<PC, PR>
where
    PC: PixClientRepository,
    PR: PixRepository,
{
    pub fn new(pix_client_repository: PC, pix_repository: PR) -> Self {
        Self {
            pix_client_repository,
            pix_repository,
        }
    }
    pub async fn execute(
        &mut self,
        input: CreatePixInput,
    ) -> Result<CreatePixOutput, CreatePixError> {
        let pix_response = self
            .pix_client_repository
            .create_pix(input.amount)
            .await
            .map_err(|e| match e {
                PixClientError::LessOrEqualZero => CreatePixError::InvalidAmount,
                _ => CreatePixError::CouldNotCreate,
            })?;

        let pix_id = PixId::from_uuid(pix_response.id());

        let pix = Pix::new(
            pix_response.amount(),
            pix_response.qr_code(),
            pix_response.qr_code_base64(),
            input.group_id.clone(),
            pix_id,
            PixStatus::Pending,
            Utc::now(),
        );

        self.pix_repository
            .create_pix(pix.clone())
            .await
            .map_err(|_| CreatePixError::CouldNotRegister)?;

        Ok(CreatePixOutput { pix: pix_response })
    }
}
