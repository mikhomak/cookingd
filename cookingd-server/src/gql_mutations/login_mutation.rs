use crate::auth::create_token;
use crate::gql_models::user_gql_model::User;
use crate::gql_mutations::LoginMutations;
use crate::guards::role::Role;
use crate::guards::role::RoleGuard;
use crate::psql_models::user_psql_model::UserModel;
use crate::utils;
use async_graphql::ErrorExtensions;
use async_graphql::{Context, FieldResult, InputObject, SimpleObject};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct LoginInfo {
    pub user: User,
    pub token: String,
}

#[async_graphql::Object]
impl LoginMutations {
    #[graphql(guard = "RoleGuard::new(Role::Anon)")]
    async fn login(&self, ctx: &Context<'_>, login_input: LoginInput) -> FieldResult<LoginInfo> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_user_model: Result<UserModel, _> =
            UserModel::find_for_email(pool, &login_input.email).await;

        let user_model = r_user_model.map_err(|e1| {
            error!("Cannot login a user with id {} due to error {}",
                        login_input.email, e1.message);
            return Err::<UserModel, async_graphql::Error>(async_graphql::Error::new("[LOGIN_001] Cannot find a user!")
                .extend_with(|_, e| e.set("error_code", "LOGIN_001")));
        }).unwrap();

        let r_token: Result<String, jsonwebtoken::errors::Error> =
            create_token(&user_model.id.to_string(), &user_model.email);
        match r_token {
            Ok(token) => Ok(LoginInfo {
                token,
                user: UserModel::convert_to_gql(&user_model),
            }),
            Err(error) => {
                error!("Cannot create a token for the user with id [{}] due to error [{}]", login_input.email.clone(), error.to_string());
                Err(
                    async_graphql::Error::new("[LOGIN_002] Cannot create a token!")
                        .extend_with(|_, e| e.set("error_code", "LOGIN_002")),
                )
            }
        }
    }
}
