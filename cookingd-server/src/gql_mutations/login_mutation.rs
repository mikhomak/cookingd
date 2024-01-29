use async_graphql::{Context, FieldResult, InputObject};
use http::header::SET_COOKIE;
use log::error;
use sqlx::PgPool;
use crate::gql_models::post_gql_model::Post;
use crate::gql_mutations::{LoginMutations, PostMutations};
use crate::gql_mutations::post_mutation::PostCreationInput;
use crate::psql_models::user_psql_model::UserModel;
use crate::guards::role::RoleGuard;
use crate::guards::role::Role;

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[async_graphql::Object]
impl LoginMutations {
    #[graphql(guard = "RoleGuard::new(Role::Anon)")]
    async fn login(
        &self,
        ctx: &Context<'_>,
        login_input: LoginInput,
    ) -> FieldResult<(bool)> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let r_user_model: Result<UserModel, _> = UserModel::read_one(pool, &login_input.email).await;
                match r_user_model {
                    Ok(user) => {
                        ctx.insert_http_header(SET_COOKIE.as_str(), "login=keke");
                        Ok((true))
                    }
                    Err(error) => {
                        error!("Cannot login a user with id {} due to error {}", login_input.email, error.message);
                        Err(async_graphql::Error::new("Cannot find a user!"))
                    }
                }
            }
            Err(_) => {
                error!("Error at logging a user. Database is not set in context!");
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }
}