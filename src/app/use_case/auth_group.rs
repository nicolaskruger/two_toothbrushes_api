use crate::domain::{
    repository::group_repository::GroupRepository, services::password_hasher::PasswordHasher,
};

pub struct AuthGroupInput {
    pub group: String,
    pub password: String,
}

pub struct AuthGroupOutput {
    pub token: String,
}

pub enum AuthGroupError {
    NotFound,
    WrongPassword,
}

pub struct AuthGroupCase<R, H>
where
    R: GroupRepository,
    H: PasswordHasher,
{
    repository: R,
    hasher: H,
}

impl<R, H> AuthGroupCase<R, H>
where
    R: GroupRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, hasher: H) -> Self {
        Self { repository, hasher }
    }

    pub async fn execute(
        &mut self,
        input: AuthGroupInput,
    ) -> Result<AuthGroupOutput, AuthGroupError> {
        let group = self
            .repository
            .find_by_name(input.group)
            .await
            .map_err(|_| AuthGroupError::NotFound)?;

        let valid = self
            .hasher
            .verify(&input.password, group.password().clone())
            .map_err(|_| AuthGroupError::WrongPassword)?;

        if valid {
            Ok(AuthGroupOutput {
                token: "".to_string(),
            })
        } else {
            Err(AuthGroupError::WrongPassword)
        }
    }
}
