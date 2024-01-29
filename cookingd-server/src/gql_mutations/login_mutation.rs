use async_graphql::{Context, FieldResult, InputObject};
use http::header::SET_COOKIE;
use log::error;
use sqlx::PgPool;
use crate::auth::create_token;
use crate::gql_mutations::LoginMutations;
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
    ) -> FieldResult<String> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let r_user_model: Result<UserModel, _> = UserModel::read_one(pool, &login_input.email).await;
                match r_user_model {
                    Ok(user) => {
                        let r_token : Result<String, jsonwebtoken::errors::Error> = create_token(&user.email);
                        match r_token
                        {
                            Ok(token) => {
                                ctx.insert_http_header(SET_COOKIE.as_str(), format!("login={}", token));
                                Ok(token)
                            },
                            Err(error)=>{
                                error!("Cannot create a token for the user with id {} due to error {}", login_input.email.clone(), error.to_string());
                                Err(async_graphql::Error::new("Cannot create a token!"))
                            }
                        }
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