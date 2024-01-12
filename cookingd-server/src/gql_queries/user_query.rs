use crate::gql_queries::user_query;
use crate::gql_queries::Query;
use async_graphql::Object;
use async_graphql::SimpleObject;
use async_graphql::Context;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use async_graphql::FieldResult;

 use crate::gql_models::user_model::User;

#[Object(extends)]
impl Query {
    async fn users<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<User>> {
        let pool = ctx.data::<PgPool>().unwrap();
        let rows = User::read_all(&pool).await?;
        Ok(rows)
    }

    async fn user<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = User::read_one(&pool, &id).await?;
        Ok(row)
    }

    #[graphql(entity)]
    async fn find_user_by_id<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = User::read_one(&pool, &id).await?;
        Ok(row)
    }
}