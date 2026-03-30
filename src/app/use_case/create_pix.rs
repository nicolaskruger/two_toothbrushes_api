use crate::domain::{
    repository::pix_client_repository::PixClientRepository,
    value_object::{group_id::GroupId, pix_response::PixResponse},
};

pub struct CreatePixInput {
    pub group_id: GroupId,
    pub amount: f64,
}

pub struct CreatePixOutput {
    pub pix: PixResponse,
}

pub struct CreatePixCase<PG>
where
    PG: PixClientRepository,
{
    pix_repository: PG,
}

#[derive(Debug, PartialEq)]
pub enum CreatePixError {
    CouldNotCreate,
}

impl<PG> CreatePixCase<PG>
where
    PG: PixClientRepository,
{
    pub fn new(pix_repository: PG) -> Self {
        Self { pix_repository }
    }
    pub async fn execute(
        &mut self,
        input: CreatePixInput,
    ) -> Result<CreatePixOutput, CreatePixError> {
        let pix = self
            .pix_repository
            .create_pix(input.amount)
            .await
            .map_err(|_| CreatePixError::CouldNotCreate)?;

        Ok(CreatePixOutput { pix })
    }
}
