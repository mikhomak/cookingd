use async_graphql::{Context, FieldResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use crate::gql_models::post_gql_model::Post;
use crate::gql_mutations::post_mutation::PostCreationInput;
use crate::main;
use crate::services::image_service;

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
    pub main_image_file_type : Option<String>
}

impl PostModel {
    pub async fn get_latest_posts(pool: &PgPool) -> FieldResult<Vec<PostModel>> {
        let r_posts: Vec<PostModel> = sqlx::query_as!(PostModel, "SELECT * FROM post ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }


    pub async fn create(pool: &PgPool,ctx: &Context<'_>, post_input: &PostCreationInput, user_id: &str) -> FieldResult<PostModel> {
        let mut main_image_type = Option::None;
        if let Some(main_image) = post_input.main_image {
            if let Ok(main_image_value) = main_image.value(ctx) {
                let image_type: String= main_image_value.content_type.unwrap().to_string();
                main_image_type = Some(image_service::map_image_type(&image_type.clone()).unwrap_or("".to_string()));
            }
        }
        let r_post : PostModel = sqlx::query_as!(
            PostModel,
            "INSERT INTO post ( user_id, title, text, rating, main_image_file_type) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            sqlx::types::Uuid::parse_str(user_id)?,
            post_input.title,
            post_input.text,
            post_input.rating,
            main_image_type.unwrap_or("".to_string())
        )
            .fetch_one(pool)
            .await?;
        Ok(r_post)
    }


    pub async fn find_posts_for_user(pool: &PgPool, user_id: &String) -> FieldResult<Vec<PostModel>> {
        let r_posts : Vec<PostModel> = sqlx::query_as!(
            PostModel,
            "SELECT * FROM post WHERE user_id = $1", sqlx::types::Uuid::parse_str(user_id)?)
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }

    pub async fn find_post_for_id(pool: &PgPool, post_id: &String) -> FieldResult<PostModel> {
        let r_post : PostModel = sqlx::query_as!(
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
            short_text: self.text.clone(),
            text: self.text.clone(),
            likes: self.likes,
            rating: self.rating,
            allow_comments: self.allow_comments,
            allow_likes: self.allow_likes,
            created_at: self.created_at,
            user_id: self.user_id,
            main_image_file_type: self.main_image_file_type.clone()
        };
    }

    pub fn convert_all_to_gql(post_models: &Vec<PostModel>) -> Vec<Post>{
        return post_models.iter().map(PostModel::convert_to_gql).collect::<Vec<Post>>();
    }
}