use anyhow::Result;
use async_graphql::{FieldResult, SimpleObject};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use chrono;
use crate::gql_mutations::user_mutation::UserRegistrationInput;

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub email: String,
    pub login_enabled: bool,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub consent: bool,
}


impl User {
    pub async fn create(pool: &PgPool, user: &UserRegistrationInput) -> FieldResult<User> {
        let r_user = sqlx::query_as!(
            User,
            "INSERT INTO c_user(name, email, password, login_enabled, consent) VALUES ($1,$2,$3,$4,$5) RETURNING *",
            user.name,
            user.email,
            user.password,
            true,
            true)
            .fetch_one(pool)
            .await?;
        Ok(r_user)

    }

    pub async fn read_one(pool: &PgPool, id: &str) -> Result<User> {
        let r_user = sqlx::query_as!(
            User,
            "SELECT * FROM c_user WHERE id = $1",
            uuid::Uuid::parse_str(id)?
        )
            .fetch_one(pool)
            .await?;

        Ok(r_user)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<User>> {
        let r_users = sqlx::query_as!(User, "SELECT * FROM c_user")
            .fetch_all(pool)
            .await?;

        Ok(r_users)
    }

    pub async fn update(pool: &PgPool, id: &str, name: &str) -> Result<User> {
        sqlx::query!(
            "UPDATE c_user SET name=$1 WHERE id = $2",
            name,
            uuid::Uuid::parse_str(id)?
        )
            .execute(pool)
            .await?;

        Ok(User::read_one(pool, id).await?)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM c_user WHERE id = $1", uuid::Uuid::parse_str(id)?)
            .execute(pool)
            .await?;

        Ok(())
    }
}