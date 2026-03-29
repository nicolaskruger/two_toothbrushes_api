use chrono::Utc;

use crate::domain::{
    entities::user::User,
    repository::{group_repository::GroupRepository, user_repository::UserRepository},
    value_object::{group_id::GroupId, pix::Pix},
};

pub struct CreatePixInput {
    pub group_id: GroupId,
    pub amount: f64,
}

pub struct CreatePixOutput {
    pub pix: Pix,
}

pub struct CreatePixCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    group_repossitor: RG,
    user_repossitor: RU,
}

#[derive(Debug, PartialEq)]
pub enum CreatePixError {
    GroupNotFound,
    CouldNotCreate,
}

impl<RG, RU> CreatePixCase<RG, RU>
where
    RG: GroupRepository,
    RU: UserRepository,
{
    pub fn new(group_repossitor: RG, user_repossitor: RU) -> Self {
        Self {
            group_repossitor,
            user_repossitor,
        }
    }
    pub async fn execute(
        &mut self,
        input: CreatePixInput,
    ) -> Result<CreatePixOutput, CreatePixError> {
        todo!()
        // let _ = self
        //     .group_repossitor
        //     .find_by_id(&GroupId::from_uuid(input.id_group))
        //     .await
        //     .map_err(|_| CreatePixError::GroupNotFound)?;
        //
        // let group_id = GroupId::from_uuid(input.id_group);
        //
        // let user = User::create(input.name.to_string(), false, group_id, Utc::now());
        //
        // self.user_repossitor
        //     .create_user(&user)
        //     .await
        //     .map_err(|_| CreatePixError::CouldNotCreate)?;
        //
        // Ok(CreatePixOutput {
        //     id: user.id().as_uuid(),
        // })
    }
}
