use std::env;
use std::path::Path;
use async_graphql::{ComplexObject, SimpleObject, Context, FieldResult };
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::gql_models::user_gql_model::User;
use crate::psql_models::user_psql_model::UserModel;
use crate::gql_models::tag_gql_model::Tag;
use crate::psql_models::tag_psql_model::TagModel;
use crate::services::image_service;

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
}

#[ComplexObject]
impl Post {
    async fn user(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<User> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_user: FieldResult<UserModel> = UserModel::read_one(pool, &self.user_id.to_string()).await;
                match r_user {
                    Ok(user_model) => Ok(UserModel::convert_to_gql(&user_model)),
                    Err(_) => Err(async_graphql::Error::new("User not found!"))
                }
            }
            Err(_) => { Err(async_graphql::Error::new("Users not found, error encountered")) }
        }
    }

    async fn tags(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Option<Vec<Tag>>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_tag_models: FieldResult<Vec<TagModel>> = TagModel::find_tags_for_post(pool, &self.id.to_string()).await;
                match r_tag_models {
                    Ok(tag_models) => Ok(Some(TagModel::convert_all_to_gql(&tag_models))),
                    Err(_) => Ok(None)
                }
            }
            Err(_) => { Err(async_graphql::Error::new("Tags not found, error encountered")) }
        }
    }

    async fn main_image_url(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Option<String>> {
                let host: String = env::var("HOST").expect("HOST is not set");
                let port: String = env::var("PORT").expect("PORT is not set");
                let r_full_url = image_service::construct_full_image_path(&self.id.to_string(), &self.user_id.to_string(), Option::None);
                match r_full_url {
                    Ok(full_url) => {
                        match Path::new(&full_url.clone()).exists() {
                            true => {
                                Ok(Some(format!("http://localhost:{}/{}", port, full_url)))
                            }
                            false => { Ok(None) }
                        }
                    }
                    Err(_) => {
                        Ok(None)
                    }
                }
    }
}