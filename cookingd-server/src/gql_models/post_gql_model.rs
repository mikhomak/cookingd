use anyhow::Error;
use async_graphql::{ComplexObject, SimpleObject};
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use async_graphql::Context;
use async_graphql::FieldResult;
use crate::psql_models::post_psql_model::PostModel;
use sqlx::PgPool;
use crate::psql_models::user_psql_model::UserModel;
use crate::gql_models::user_gql_model::User;

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Post {
    #[graphql(skip)]
    pub user_id: sqlx::types::Uuid,
    pub id: sqlx::types::Uuid,
    pub title: String,
    pub text: Option<String>,
    pub likes: i64,
    pub rating: f64,
    pub allow_comments: bool,
    pub allow_likes: bool,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl Post {
    async fn user(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<User> {
        let r_pool: std::result::Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_user: Result<UserModel, _> = UserModel::read_one(pool, &self.user_id.to_string()).await;
                match r_user {
                    Ok(user_model) => Ok(UserModel::convert_to_gql(&user_model)),
                    Err(_) => Err(async_graphql::Error::new("User not found!"))
                }
            }
            Err(_) => { Err(async_graphql::Error::new("Users not found, error encountered")) }
        }
    }
}