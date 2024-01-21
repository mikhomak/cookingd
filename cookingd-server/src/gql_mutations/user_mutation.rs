use anyhow::Result;
use async_graphql::{Context,  FieldResult,  ID, InputObject};
use sqlx::postgres::PgPool;
use log::{error, info};
use crate::gql_mutations::UserMutations;

use crate::psql_models::user_psql_model::UserModel;
use crate::servies::site_configuration_service::is_registration_enabled;
use crate::gql_models::user_gql_model::User;


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

        match r_pool {
            Ok(pool) => {
                let is_registration_enabled: bool = is_registration_enabled(pool).await;
                if is_registration_enabled == false {
                    return Err(async_graphql::Error::new("Registration failed!"));
                }

                let r_created_user : FieldResult<UserModel> = UserModel::create(&pool, &user_input).await;

                match r_created_user {
                    Ok(created_user) => Ok(UserModel::convert_to_gql(&created_user)),
                    Err(_) => {
                        error!("Cannot create a user due to error");
                        Err(async_graphql::Error::new("Registration failed!"))
                    }
                }
            }
            Err(_) => {
                error!("Error at creating a user. Database is not set in context!");
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let r_pool : Result<&PgPool,_>= ctx.data::<PgPool>();
        let id : String = id.parse::<String>()?;
        match r_pool {
            Ok(pool) => {
                let r_delete: Result<(), _> = UserModel::delete(&pool, &id).await;
                match r_delete {
                    Ok(_) => {
                        info!("User with id {} was deleted", id);
                        Ok(true)
                    }
                    Err(error) => {
                        error!("Cannot delete a user with id {} due to error {}", id, error.to_string());
                        Err(async_graphql::Error::new("Something failed!"))
                    }
                }
            }
            Err(_) => {
                error!("Error at deleting a user with id {}. Database is not set in context!", id);
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
    ) -> FieldResult<User> {
        let id = id.parse::<String>()?;
        let r_pool = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let r_user: FieldResult<UserModel> = UserModel::update(&pool, &id, &name).await;
                match r_user {
                    Ok(user) => {
                        info!("User with id [{}] was updated!", id);
                        Ok(UserModel::convert_to_gql(&user))
                    }
                    Err(error) => {
                        error!("Cannot update a user with id {} due to error {}", id, error.to_string());
                        Err(async_graphql::Error::new("Cannot update a user!"))
                    }
                }
            }
            Err(_) => {
                error!("Error at updating a user with id {}. Database is not set in context!", id);
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }
}