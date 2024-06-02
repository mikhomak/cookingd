use async_graphql::{ComplexObject, Context, FieldResult, SimpleObject};
use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::gql_models::post_gql_model::{Post, PostsPagination};
use crate::psql_models::post_psql_model::PostModel;
use crate::utils;

#[derive(SimpleObject, Deserialize, Serialize)]
#[graphql(complex)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub email: String,
    pub login_enabled: bool,
    #[graphql(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub consent: bool,
}

#[ComplexObject]
impl User {
    async fn posts(&self, ctx: &Context<'_>) -> FieldResult<PostsPagination> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_posts: FieldResult<Vec<PostModel>> =
            PostModel::find_posts_for_user(pool, &self.id.to_string()).await;
        let pages: i64 = PostModel::count_posts(pool).await;
        match r_posts {
            Ok(post_models) => Ok(PostsPagination {
                posts: PostModel::convert_all_to_gql(&post_models),
                pages
            }),
            Err(_) => Err(async_graphql::Error::new("Posts not found!")),
        }
    }
}
