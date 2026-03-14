use crate::domain::{
    repository::{auth_repository::AuthRepository, group_repository::GroupRepository},
    services::password_hasher::PasswordHasher,
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
    GenTokenError,
}

pub struct AuthGroupCase<R, H, A>
where
    R: GroupRepository,
    H: PasswordHasher,
    A: AuthRepository,
{
    repository: R,
    hasher: H,
    auth: A,
}

impl<R, H, A> AuthGroupCase<R, H, A>
where
    R: GroupRepository,
    H: PasswordHasher,
    A: AuthRepository,
{
    pub fn new(repository: R, hasher: H, auth: A) -> Self {
        Self {
            repository,
            hasher,
            auth,
        }
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
                token: self
                    .auth
                    .token(&group)
                    .await
                    .map_err(|_| AuthGroupError::GenTokenError)?,
            })
        } else {
            Err(AuthGroupError::WrongPassword)
        }
    }
}
