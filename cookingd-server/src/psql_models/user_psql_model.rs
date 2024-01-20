use anyhow::Result;
use async_graphql::FieldResult;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use chrono;
use crate::gql_models::user_gql_model::User;
use crate::gql_mutations::user_mutation::UserRegistrationInput;

#[derive(FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub email: String,
    pub login_enabled: bool,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub consent: bool,
}

impl UserModel {
    pub async fn create(pool: &PgPool, user: &UserRegistrationInput) -> FieldResult<UserModel> {
        let r_user : FieldResult<UserModel> = sqlx::query_as!(
            UserModel,
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

    pub async fn read_one(pool: &PgPool, id: &str) -> Result<UserModel> {
        let r_user : FieldResult<UserModel> = sqlx::query_as!(
            UserModel,
            "SELECT * FROM c_user WHERE id = $1",
            uuid::Uuid::parse_str(id)?
        )
            .fetch_one(pool)
            .await?;

        Ok(r_user)
    }

    pub async fn read_all(pool: &PgPool) -> Result<Vec<UserModel>> {
        let r_users : FieldResult<Vec<UserModel>> = sqlx::query_as!(UserModel, "SELECT * FROM c_user")
            .fetch_all(pool)
            .await?;

        Ok(r_users)
    }

    pub async fn update(pool: &PgPool, id: &str, name: &str) -> Result<UserModel> {
        sqlx::query!(
            "UPDATE c_user SET name=$1 WHERE id = $2",
            name,
            uuid::Uuid::parse_str(id)?
        )
            .execute(pool)
            .await?;

        Ok(UserModel::read_one(pool, id).await?)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM c_user WHERE id = $1", uuid::Uuid::parse_str(id)?)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub fn convert_to_gql(&self) -> User {
        User {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            login_enabled: self.login_enabled,
            password: self.password.clone(),
            created_at: self.created_at,
            consent: self.consent,
        }
    }

    pub fn convert_all_to_gql(user_models: &Vec<UserModel>) -> Vec<User> {
        user_models.iter().map(UserModel::convert_to_gql).collect::<Vec<User>>()
    }
}