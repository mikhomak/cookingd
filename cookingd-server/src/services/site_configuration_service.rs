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
    let r_allow_registration: Result<bool, _> =
        sqlx::query_scalar!("SELECT allow_registration FROM site_configuration")
            .fetch_one(pg_pool)
            .await;

    r_allow_registration.unwrap_or_else(|_| false)
}

pub async fn is_posting_allowed(pg_pool: &PgPool) -> bool {
    let r_allow_posting: Result<bool, _> =
        sqlx::query_scalar!("SELECT allow_posting FROM site_configuration")
            .fetch_one(pg_pool)
            .await;

    r_allow_posting.unwrap_or_else(|_| false)
}
