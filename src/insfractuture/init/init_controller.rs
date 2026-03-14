use crate::{
    app::use_case::{
        create_group::{CreateGroupCase, CreateGroupInput},
        create_user::{CreateUserCase, CreateUserInput},
    },
    domain::{
        repository::{group_repository::GroupRepository, user_repository::UserRepository},
        services::password_hasher::PasswordHasher,
    },
    insfractuture::config::settings::Settings,
};

pub async fn init_group<RG: GroupRepository, RU: UserRepository, H: PasswordHasher>(
    settings: &Settings,
    mut create_group_case: CreateGroupCase<RG, H>,
    mut create_user_case: CreateUserCase<RG, RU>,
    mut group_repository: RG,
    mut user_repository: RU,
) {
    if group_repository.count().await.unwrap() == 0 && user_repository.count().await.unwrap() == 0 {
        for gp in settings.group_list.iter().clone() {
            let group = CreateGroupInput {
                name: gp.name.clone(),
                password: gp.password.clone(),
            };

            let group = create_group_case.execute(group).await.unwrap();

            for us in gp.users.iter().clone() {
                let user = CreateUserInput {
                    name: us.clone(),
                    id_group: group.id,
                };

                create_user_case.execute(user).await.unwrap();
            }
        }
    }
}
