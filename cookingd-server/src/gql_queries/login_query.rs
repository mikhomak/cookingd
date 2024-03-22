use crate::auth::{get_token, CookingdClaims};
use crate::gql_mutations::login_mutation::LoginInfo;
use crate::gql_queries::LoginQuery;
use crate::guards::role::Role;
use crate::guards::role::RoleGuard;
use crate::psql_models::user_psql_model::UserModel;
use async_graphql::{Context, ErrorExtensions, FieldResult, Object};
use jsonwebtoken::TokenData;
use log::error;
use sqlx::PgPool;
#[Object(extends)]
impl LoginQuery {
    #[graphql(guard = "RoleGuard::new(Role::Anon)")]
    async fn verify_token<'a>(
        &self,
        ctx: &'a Context<'_>,
        token: String,
    ) -> FieldResult<LoginInfo> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let r_token: Result<TokenData<CookingdClaims>, jsonwebtoken::errors::Error> =
                    get_token(&token);
                match r_token {
                    Ok(token_data) => {
                        let email: String = token_data.claims.email;
                        let r_user_model: Result<UserModel, _> =
                            UserModel::find_for_email(pool, &email).await;
                        match r_user_model {
                            Ok(user_model) => Ok(LoginInfo {
                                token,
                                user: UserModel::convert_to_gql(&user_model),
                            }),
                            Err(error) => {
                                error!(
                                    "Cannot create a token for the user with id {} due to error {}",
                                    email.clone(),
                                    error.message
                                );
                                Err(async_graphql::Error::new(
                                    "[LOGIN_003] Cannot find a user from token!",
                                )
                                .extend_with(|_, e| e.set("error_code", "LOGIN_003")))
                            }
                        }
                    }
                    Err(_) => Err(
                        async_graphql::Error::new("[LOGIN_004] Cannot verify the token")
                            .extend_with(|_, e| e.set("error_code", "LOGIN_004")),
                    ),
                }
            }
            Err(_) => {
                error!("Error at logging a user. Database is not set in context!");
                Err(async_graphql::Error::new("[SERVER_001] Server error!")
                    .extend_with(|_, e| e.set("error_code", "SERVER_001")))
            }
        }
    }
}
