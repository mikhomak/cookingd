
use actix_web::{guard, middleware, web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema, ID};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;

use crate::gql_mutations::Mutations;

use crate::gql_models::user_model::User;


#[async_graphql::Object]
impl Mutations {
/*    async fn create_User(
        &self,
        ctx: &Context<'_>,
        title: String,
    ) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = User::create(&pool).await?;
        Ok(row)
    }*/

    async fn delete_User(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        User::delete(&pool, &id).await?;
        Ok(true)
    }

    async fn update_User(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String
    ) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let id = id.parse::<String>()?;

        let row = User::update(&pool, &id, &name).await?;
        Ok(row)
    }
}