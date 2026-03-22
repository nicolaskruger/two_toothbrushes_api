use crate::domain::{
    entities::group::Group, repository::group_repository::GroupRepository,
    value_object::group_id::GroupId,
};

pub struct FindGroupByIdInput {
    pub group_id: GroupId,
}

pub struct FindGroupByIdOutput {
    pub group: Group,
}

pub enum FindGroupByIdError {
    ThisGroupNotExists,
}

pub struct FindGroupByIdCase<R>
where
    R: GroupRepository,
{
    repository: R,
}

impl<R> FindGroupByIdCase<R>
where
    R: GroupRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &mut self,
        input: FindGroupByIdInput,
    ) -> Result<FindGroupByIdOutput, FindGroupByIdError> {
        let group = self
            .repository
            .find_by_id(&input.group_id)
            .await
            .map_err(|_| FindGroupByIdError::ThisGroupNotExists)?;

        let output = FindGroupByIdOutput { group };

        Ok(output)
    }
}
