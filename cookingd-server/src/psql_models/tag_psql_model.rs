use actix_web::web::post;
use async_graphql::{FieldResult, SimpleObject};
use async_graphql::futures_util::future::err;
use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool, QueryBuilder, Row};
use sqlx::postgres::{PgQueryResult, PgRow};
use crate::gql_models::post_gql_model::Post;
use crate::gql_models::tag_gql_model::Tag;
use crate::gql_mutations::post_mutation::TagCreationInput;

#[derive(FromRow, Deserialize, Serialize)]
pub struct TagModel {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl TagModel {
    pub async fn create(pool: &PgPool, tag_input: &TagCreationInput) -> FieldResult<TagModel> {
        let tag_model = sqlx::query_as!(
            TagModel,
            "INSERT INTO tag (name) VALUES ($1) RETURNING *",
            tag_input.name)
            .fetch_one(pool)
            .await?;
        Ok(tag_model)
    }


    pub async fn create_batch_tags(pool: &PgPool, tag_names: &Vec<String>) -> FieldResult<Vec<sqlx::types::Uuid>> {
        let mut query_builder = QueryBuilder::new("INSERT INTO tag (name) ");

        query_builder.push_values(tag_names, |mut b, tag_id: &String| {
            b.push_bind(tag_id);
        });

        query_builder.push(" returning *");
        let query = query_builder.build();


        let r_tag_ids = query.fetch_all(pool).await?;
        Ok(r_tag_ids
            .iter()
            .map(|row| row.get::<sqlx::types::Uuid, _>("id"))
            .collect())
    }

    pub async fn associate_tags_to_post(pool: &PgPool, tag_ids: &Vec<sqlx::types::Uuid>, post_id: &sqlx::types::Uuid) -> PgQueryResult {
        let mut query_builder = QueryBuilder::new("INSERT INTO tag_to_post (tag_id, post_id) ");

        query_builder.push_values(tag_ids, |mut b, tag_id: &sqlx::types::Uuid| {
            b.push_bind(tag_id).push_bind(post_id);
        });

        let query = query_builder.build();

        query.execute(pool).await.unwrap()
    }

    pub async fn find_tags_for_post(pool: &PgPool, post_id: &String) -> FieldResult<Vec<TagModel>> {
        let r_posts = sqlx::query_as!(
            TagModel,
            "SELECT t.* FROM (tag AS t LEFT JOIN tag_to_post as t2l ON t.id = t2l.tag_id) WHERE t2l.post_id = $1",
            sqlx::types::Uuid::parse_str(post_id)?)
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }

    pub async fn find_tags_for_name(pool: &PgPool, name: &String) -> FieldResult<Vec<TagModel>> {
        let r_tags = sqlx::query_as!(
            TagModel,
            "SELECT * FROM tag WHERE name LIKE $1",
            format!("%{}%", name))
            .fetch_all(pool)
            .await?;
        Ok(r_tags)
    }

    pub fn convert_to_gql(&self) -> Tag {
        return Tag {
            id: self.id,
            name: self.name.clone(),
            created_at: self.created_at,
        };
    }

    pub fn convert_all_to_gql(post_models: &Vec<TagModel>) -> Vec<Tag> {
        return post_models.iter().map(TagModel::convert_to_gql).collect::<Vec<Tag>>();
    }
}