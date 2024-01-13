use crate::gql_queries::PostQuery;
use async_graphql::Object;
use async_graphql::Context;
use sqlx::PgPool;
use async_graphql::FieldResult;
use log::error;
use crate::gql_models::post_gql_model::Post;
use crate::psql_models::post_psql_model::PostModel;


#[Object(extends)]
impl PostQuery {
    async fn latest_posts<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Post>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_posts = PostModel::get_latest_posts(&pool).await;
                match r_posts {
                    Ok(posts) => Ok(PostModel::convert_all_to_gql(&posts)),
                    Err(error) => {
                        error!("Posts couldn't be fetched from the db due to error");
                        Err(async_graphql::Error::new("Posts not found, error encountered"))
                    }
                }
            }
            Err(_) => {
                error!("Database is not set up in the context");
                Err(async_graphql::Error::new("Server Error!"))
            }
        }
    }

    async fn posts_for_user<'a>(&self, ctx: &'a Context<'_>, user_id: String) -> FieldResult<Vec<Post>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_posts = PostModel::find_posts_for_user(&pool, &user_id).await;
                match r_posts {
                    Ok(posts) => Ok(PostModel::convert_all_to_gql(&posts)),
                    Err(error) => {
                        error!("Posts for user with id {} not found due to error", user_id);
                        Err(async_graphql::Error::new("Posts not found, error encountered"))
                    }
                }
            }
            Err(_) => {
                error!("Database is not set up in the context");
                Err(async_graphql::Error::new("Server Error!"))
            }
        }
    }
}