use async_graphql::FieldResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use crate::gql_models::post_gql_model::Post;
use crate::gql_mutations::post_mutation::PostCreationInput;

#[derive(FromRow, Deserialize, Serialize)]
pub struct PostModel {
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

impl PostModel {
    pub async fn get_latest_posts(pool: &PgPool) -> FieldResult<Vec<PostModel>> {
        let r_posts: FieldResult<Vec<PostModel>> = sqlx::query_as!(PostModel, "SELECT * FROM post ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }


    pub async fn create(pool: &PgPool, post_input: &PostCreationInput) -> FieldResult<PostModel> {
        let r_post : FieldResult<PostModel> = sqlx::query_as!(
            PostModel,
            "INSERT INTO post ( user_id, title, text, rating) VALUES ($1, $2, $3, $4) RETURNING *",
            sqlx::types::Uuid::parse_str(&post_input.user_id)?,
            post_input.title,
            post_input.text,
            post_input.rating)
            .fetch_one(pool)
            .await?;
        Ok(r_post)
    }


    pub async fn find_posts_for_user(pool: &PgPool, user_id: &String) -> FieldResult<Vec<PostModel>> {
        let r_posts : FieldResult<Vec<PostModel>> = sqlx::query_as!(
            PostModel,
            "SELECT * FROM post WHERE user_id = $1", sqlx::types::Uuid::parse_str(user_id)?)
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }

    pub async fn find_post_for_id(pool: &PgPool, post_id: &String) -> FieldResult<PostModel> {
        let r_post : FieldResult<PostModel> = sqlx::query_as!(
            PostModel,
            "SELECT * FROM post WHERE id = $1", sqlx::types::Uuid::parse_str(post_id)?)
            .fetch_one(pool)
            .await?;
        Ok(r_post)
    }

    pub fn convert_to_gql(&self) -> Post {
        return Post {
            id: self.id,
            title: self.title.clone(),
            text: self.text.clone(),
            likes: self.likes,
            rating: self.rating,
            allow_comments: self.allow_comments,
            allow_likes: self.allow_likes,
            created_at: self.created_at,
            user_id: self.user_id
        };
    }

    pub fn convert_all_to_gql(post_models: &Vec<PostModel>) -> Vec<Post>{
        return post_models.iter().map(PostModel::convert_to_gql).collect::<Vec<Post>>();
    }
}