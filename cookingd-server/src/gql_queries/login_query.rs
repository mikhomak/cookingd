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
use crate::utils;

#[Object(extends)]
impl LoginQuery {

    #[graphql(guard = "RoleGuard::new(Role::Anon)")]
    async fn verify_token<'a>(
        &self,
        ctx: &'a Context<'_>,
        token: String,
    ) -> FieldResult<LoginInfo> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_token: Result<TokenData<CookingdClaims>, jsonwebtoken::errors::Error> =
            get_token(&token);

        let Ok(token_data) = r_token else {
            return Err(async_graphql::Error::new("[LOGIN_004] Cannot verify the token")
                .extend_with(|_, e| e.set("error_code", "LOGIN_004")));
        };

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
                ).extend_with(|_, e| e.set("error_code", "LOGIN_003")))
            }
        }
    }

}
