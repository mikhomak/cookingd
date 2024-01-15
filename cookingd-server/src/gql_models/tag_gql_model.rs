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

#[derive(SimpleObject, Deserialize, Serialize)]
pub struct Tag {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
