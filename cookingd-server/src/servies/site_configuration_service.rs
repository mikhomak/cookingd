use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Deserialize, Serialize)]
struct SiteConfiguration {
    id: i32,
    allow_site_comments: bool,
    allow_posting: bool,
    allow_registration: bool,
    allow_login: bool,
}

pub async fn is_registration_enabled(pg_pool: &PgPool) -> bool {
    let r_site_configuration = sqlx::query_as!(
        SiteConfiguration,
        "SELECT * FROM site_configuration")
        .fetch_one(pg_pool)
        .await;

    match r_site_configuration {
        Ok(site_configuration) => site_configuration.allow_registration,
        Err(_) => false
    }
}