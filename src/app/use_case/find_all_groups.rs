use crate::domain::{entities::group::Group, repository::group_repository::GroupRepository};

pub struct FindAllGroupsOutput {
    pub groups: Vec<Group>,
}

#[derive(Debug, PartialEq)]
pub enum FindAllGroupsError {
    CouldNotList,
}

pub struct FindAllGroupsCase<R>
where
    R: GroupRepository,
{
    repository: R,
}

impl<R> FindAllGroupsCase<R>
where
    R: GroupRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&mut self) -> Result<FindAllGroupsOutput, FindAllGroupsError> {
        let groups = self
            .repository
            .find_all()
            .await
            .map_err(|_| FindAllGroupsError::CouldNotList)?;

        Ok(FindAllGroupsOutput { groups })
    }
}
