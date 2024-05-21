use crate::gql_mutations::UserMutations;
use anyhow::Result;
use async_graphql::{Context, FieldResult, InputObject, ID};
use log::{error, info};
use sqlx::postgres::PgPool;

use crate::gql_models::user_gql_model::User;
use crate::psql_models::user_psql_model::UserModel;
use crate::services::site_configuration_service::is_registration_enabled;
use crate::utils;

#[derive(InputObject)]
pub struct UserRegistrationInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub consent: bool,
}

#[async_graphql::Object]
impl UserMutations {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        user_input: UserRegistrationInput,
    ) -> FieldResult<User> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let is_registration_enabled: bool = is_registration_enabled(pool).await;
        if is_registration_enabled == false {
            return Err(async_graphql::Error::new("Registration failed!"));
        }

        let r_created_user: FieldResult<UserModel> =
            UserModel::create(&pool, &user_input).await;

        match r_created_user {
            Ok(created_user) => Ok(UserModel::convert_to_gql(&created_user)),
            Err(_) => {
                error!("Cannot create a user due to error");
                Err(async_graphql::Error::new("Registration failed!"))
            }
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, _> = ctx.data::<PgPool>();
        let id: String = id.parse::<String>()?;

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_delete: Result<(), _> = UserModel::delete(&pool, &id).await;
        match r_delete {
            Ok(_) => {
                info!("User with id {} was deleted", id);
                Ok(true)
            }
            Err(error) => {
                error!(
                            "Cannot delete a user with id {} due to error {}",
                            id, error.message
                        );
                Err(async_graphql::Error::new("Something failed!"))
            }
        }
    }

    async fn update_user(&self, ctx: &Context<'_>, id: ID, name: String) -> FieldResult<User> {
        let id = id.parse::<String>()?;
        let r_pool = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_user: FieldResult<UserModel> = UserModel::update(&pool, &id, &name).await;
        match r_user {
            Ok(user) => {
                info!("User with id [{}] was updated!", id);
                Ok(UserModel::convert_to_gql(&user))
            }
            Err(error) => {
                error!("Cannot update a user with id {} due to error {}", id, error.message);
                Err(async_graphql::Error::new("Cannot update a user!"))
            }
        }
    }
}
