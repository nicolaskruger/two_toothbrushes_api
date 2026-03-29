use crate::domain::{
    repository::pix_repository::PixRepository,
    value_object::{group_id::GroupId, pix::Pix},
};

pub struct CreatePixInput {
    pub group_id: GroupId,
    pub amount: f64,
}

pub struct CreatePixOutput {
    pub pix: Pix,
}

pub struct CreatePixCase<PG>
where
    PG: PixRepository,
{
    pix_repository: PG,
}

#[derive(Debug, PartialEq)]
pub enum CreatePixError {
    CouldNotCreate,
}

impl<PG> CreatePixCase<PG>
where
    PG: PixRepository,
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
