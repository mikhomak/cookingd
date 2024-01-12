use anyhow::Result;
use async_graphql::SimpleObject;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: sqlx::types::Uuid,
    name: String,
    email: String,
    login_enabled: bool,
    password: String,
    created_at: DateTime<Utc>,
    consent: bool,
}


impl User {
    /*
    pub async fn create(pool: &PgPool, name: &str) -> Result<User> {
        let row = sqlx::query!(
            "INSERT INTO user(name) VALUES ($1) RETURNING id",
            name,
        )
            .fetch_one(pool)
            .await?;

        Ok(row)
    }
     */

    pub async fn read_one(pool: &PgPool, id: &str) -> Result<User> {
        let row = sqlx::query_as!(
            User,
            "SELECT * FROM \"user\" WHERE id = $1",
            Uuid::parse_str(id)?
        )
            .fetch_one(pool)
            .await?;

        Ok(row)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<User>> {
        let rows = sqlx::query_as!(User, "SELECT * FROM \"user\"")
            .fetch_all(pool)
            .await?;

        Ok(rows)
    }

    pub async fn update(pool: &PgPool, id: &str, name: &str) -> Result<User> {
        sqlx::query!(
            "UPDATE \"user\" SET name=$1 WHERE id = $2",
            name,
            Uuid::parse_str(id)?
        )
            .execute(pool)
            .await?;

        Ok(User::read_one(pool, id).await?)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM \"user\" WHERE id = $1", Uuid::parse_str(id)?)
            .execute(pool)
            .await?;

        Ok(())
    }
}