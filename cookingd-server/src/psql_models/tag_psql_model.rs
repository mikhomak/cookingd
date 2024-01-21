use async_graphql::FieldResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, QueryBuilder, Row};
use sqlx::postgres::PgQueryResult;
use crate::gql_models::tag_gql_model::Tag;

#[derive(FromRow, Deserialize, Serialize)]
pub struct TagModel {
    pub name: String,
    pub user_who_created: Option<sqlx::types::Uuid>,
    pub created_at: DateTime<Utc>,
}

impl TagModel {
    pub async fn create(pool: &PgPool, name: &String) -> FieldResult<TagModel> {
        let tag_model : FieldResult<TagModel> = sqlx::query_as!(
            TagModel,
            "INSERT INTO tag (name) VALUES ($1) RETURNING *",
            name.to_lowercase())
            .fetch_one(pool)
            .await?;
        Ok(tag_model)
    }

    pub async fn get_all(pool: &PgPool) -> FieldResult<Vec<TagModel>> {
        let r_tag_models : FieldResult<Vec<TagModel>> = sqlx::query_as!(
            TagModel,
            "SELECT * FROM tag")
            .fetch_all(pool)
            .await?;
        Ok(r_tag_models)
    }

    pub async fn create_batch_tags(pool: &PgPool, tag_names: &Vec<String>) -> FieldResult<Vec<String>> {
        let mut query_builder : FieldResult<Vec<String>> = QueryBuilder::new("WITH created_tag AS (INSERT INTO tag (name) ");

        query_builder.push_values(tag_names, |mut b, tag_id: &String| {
            b.push_bind(tag_id);
        });

        query_builder.push(" ON CONFLICT DO NOTHING RETURNING *),");
        query_builder.push(" existed_tag as (SELECT * FROM tag WHERE name IN(");
        let mut separated = query_builder.separated(", ");
        for tag_name in tag_names.iter() {
            separated.push_bind(tag_name.to_lowercase());
        }
        separated.push_unseparated(") ");
        query_builder.push(") SELECT * FROM created_tag UNION ALL SELECT * FROM existed_tag");
        let query = query_builder.build();

        let r_created_tag_names = query.fetch_all(pool).await?;
        Ok(r_created_tag_names
            .iter()
            .map(|row| row.get::<String, _>("name"))
            .collect())
    }

    pub async fn associate_tags_to_post(pool: &PgPool, tag_names: &Vec<String>, post_id: &sqlx::types::Uuid) -> PgQueryResult {
        let mut query_builder = QueryBuilder::new("INSERT INTO tag_to_post (tag_name, post_id) ");

        query_builder.push_values(tag_names, |mut b, tag_name: &String| {
            b.push_bind(tag_name).push_bind(post_id);
        });
        query_builder.push("ON CONFLICT DO NOTHING");

        let query = query_builder.build();

        query.execute(pool).await.unwrap()
    }


    pub async fn remove_tag_association(pool: &PgPool, tag_names: &Vec<String>, post_id: &sqlx::types::Uuid) -> PgQueryResult {
        let mut query_builder = QueryBuilder::new("DELETE FROM tag_to_post WHERE post_id = ");
        query_builder.push_bind(post_id);
        query_builder.push(" AND tag_name IN (");
        let mut separated = query_builder.separated(", ");
        for tag_name in tag_names.iter() {
            separated.push_bind(tag_name.to_lowercase());
        }
        separated.push_unseparated(") ");

        let query = query_builder.build();

        query.execute(pool).await.unwrap()
    }


    pub async fn find_tags_for_post(pool: &PgPool, post_id: &String) -> FieldResult<Vec<TagModel>> {
        let r_posts : FieldResult<Vec<TagModel>> = sqlx::query_as!(
            TagModel,
            "SELECT t.* FROM (tag AS t LEFT JOIN tag_to_post as t2l ON t.name = t2l.tag_name) WHERE t2l.post_id = $1",
            sqlx::types::Uuid::parse_str(post_id)?)
            .fetch_all(pool)
            .await?;
        Ok(r_posts)
    }

    pub async fn find_tags_for_name(pool: &PgPool, name: &String) -> FieldResult<Vec<TagModel>> {
        let r_tags : FieldResult<Vec<TagModel>> = sqlx::query_as!(
            TagModel,
            "SELECT * FROM tag WHERE name LIKE $1",
            format!("%{}%", name))
            .fetch_all(pool)
            .await?;
        Ok(r_tags)
    }

    pub fn convert_to_gql(&self) -> Tag {
        return Tag {
            name: self.name.clone(),
            created_at: self.created_at,
        };
    }

    pub fn convert_all_to_gql(post_models: &Vec<TagModel>) -> Vec<Tag> {
        return post_models.iter().map(TagModel::convert_to_gql).collect::<Vec<Tag>>();
    }
}