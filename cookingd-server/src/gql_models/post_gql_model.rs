use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use std::path::Path;

use crate::gql_models::tag_gql_model::Tag;
use crate::gql_models::user_gql_model::User;
use crate::psql_models::tag_psql_model::TagModel;
use crate::psql_models::user_psql_model::UserModel;
use crate::services::image_service;
use crate::utils;

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct Post {
    #[graphql(skip)]
    pub user_id: sqlx::types::Uuid,
    pub id: sqlx::types::Uuid,
    pub title: String,
    pub short_text: Option<String>,
    pub text: Option<String>,
    pub likes: i64,
    pub rating: f64,
    pub allow_comments: bool,
    pub allow_likes: bool,
    pub created_at: DateTime<Utc>,
    #[graphql(skip)]
    pub main_image_file_type: Option<String>,
}

#[ComplexObject]
impl Post {
    async fn user(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_user: FieldResult<UserModel> =
            UserModel::read_one(pool, &self.user_id.to_string()).await;
        match r_user {
            Ok(user_model) => Ok(UserModel::convert_to_gql(&user_model)),
            Err(_) => Err(async_graphql::Error::new("User not found!")),
        }
    }

    async fn tags(&self, ctx: &Context<'_>) -> FieldResult<Option<Vec<Tag>>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_tag_models: FieldResult<Vec<TagModel>> =
            TagModel::find_tags_for_post(pool, &self.id.to_string()).await;
        match r_tag_models {
            Ok(tag_models) => Ok(Some(TagModel::convert_all_to_gql(&tag_models))),
            Err(_) => Ok(None),
        }
    }


    async fn main_image_url(&self, _ctx: &Context<'_>) -> FieldResult<Option<String>> {
        let backend_url: String = env::var("BACKEND_URL").expect("BACKEND_URL is not set");
        let image_type = self
            .main_image_file_type
            .clone()
            .map_or_else(|| "jpeg".to_string(), |file_type| file_type);
        let r_full_url = image_service::construct_full_image_path(
            &self.id.to_string(),
            &self.user_id.to_string(),
            Some(image_type.as_str()),
            "images",
        );
        let f_image_dir: String = dotenv::var("IMAGES_DIR").unwrap_or("images/".to_string());
        let r_full_dir = image_service::construct_full_image_path(
            &self.id.to_string(),
            &self.user_id.to_string(),
            Some(image_type.as_str()),
            f_image_dir.as_str(),
        );
        match r_full_url {
            Ok(full_url) => match Path::new(&r_full_dir.unwrap()).exists() {
                true => Ok(Some(format!(
                    "{}/{}",
                    backend_url, full_url
                ))),
                false => Ok(None),
            },
            Err(_) => Ok(None),
        }
    }
}
