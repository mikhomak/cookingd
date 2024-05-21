use crate::gql_models::user_gql_model::User;
use crate::gql_queries::UserQuery;
use crate::psql_models::user_psql_model::UserModel;
use async_graphql::Context;
use async_graphql::FieldResult;
use async_graphql::Object;
use log::error;
use sqlx::PgPool;
use crate::utils;

#[Object(extends)]
impl UserQuery {
    async fn users<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<User>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_users: FieldResult<Vec<UserModel>> = UserModel::read_all(&pool).await;
        match r_users {
            Ok(users) => Ok(UserModel::convert_all_to_gql(&users)),
            Err(error) => {
                error!(
                            "Users couldn't be fetched from the db due to error {}",
                            error.message
                        );
                Err(async_graphql::Error::new(
                    "Users not found, error encountered",
                ))
            }
        }
    }

    async fn user<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<User> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_user: FieldResult<UserModel> = UserModel::read_one(&pool, &id).await;
        match r_user {
            Ok(users) => Ok(UserModel::convert_to_gql(&users)),
            Err(error) => {
                error!(
                            "User with id {} not found due to error {}",
                            id, error.message
                        );
                Err(async_graphql::Error::new(
                    "User not found, error encountered",
                ))
            }
        }
    }
}