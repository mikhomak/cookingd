use async_graphql::{FieldResult, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use crate::gql_mutations::post_mutation::PostCreationInput;

#[derive(SimpleObject, FromRow, Deserialize, Serialize)]
pub struct Post {
    pub id: sqlx::types::Uuid,
    pub user_id: sqlx::types::Uuid,
    pub title: String,
    pub text: Option<String>,
    pub likes: i64,
    pub rating: f64,
    pub allow_comments: bool,
    pub allow_likes: bool,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub async fn get_latest_posts(pool: &PgPool) -> FieldResult<Vec<Post>> {
        let r_posts = sqlx::query_as!(Post, "SELECT * FROM post ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }


    pub async fn create(pool: &PgPool, post_input: &PostCreationInput) -> FieldResult<Post> {
        let r_post = sqlx::query_as!(
            Post,
            "INSERT INTO post ( user_id, title, text, rating) VALUES ($1, $2, $3, $4) RETURNING *",
            sqlx::types::Uuid::parse_str(&post_input.user_id)?,
            post_input.title,
            post_input.text,
            post_input.rating)
            .fetch_one(pool)
            .await?;
        Ok(r_post)
    }


    pub async fn find_posts_for_user(pool: &PgPool, user_id: &String) -> FieldResult<Vec<Post>> {
        let r_posts = sqlx::query_as!(
            Post,
            "SELECT * FROM post WHERE user_id = $1", sqlx::types::Uuid::parse_str(user_id)?)
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }
}