use actix_web::{guard, middleware, web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema, ID, InputObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;

use crate::gql_mutations::Mutations;

use crate::gql_models::user_model::User;
use crate::servies::site_configuration_service::is_registration_enabled;

#[derive(InputObject)]
pub struct UserRegistrationInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub consent: bool,
}

#[async_graphql::Object]
impl Mutations {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        user_input: UserRegistrationInput,
    ) -> FieldResult<User> {
        let pool: &PgPool = ctx.data::<PgPool>().unwrap();

        let is_registration_enabled: bool = is_registration_enabled(pool).await;
        if is_registration_enabled == false {
            return Err(async_graphql::Error::new("Registration failed!"));
        }

        let row = User::create(&pool, &UserRegistrationInput {
            name: user_input.name,
            email: user_input.email,
            password: user_input.password,
            consent: true,
        }).await?;

        Ok(row)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        User::delete(&pool, &id).await?;
        Ok(true)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
    ) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        let row = User::update(&pool, &id, &name).await?;
        Ok(row)
    }
}