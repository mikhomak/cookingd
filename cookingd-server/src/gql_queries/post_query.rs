use crate::gql_models::post_gql_model::Post;
use crate::gql_queries::PostQuery;
use crate::psql_models::post_psql_model::PostModel;
use async_graphql::Context;
use async_graphql::FieldResult;
use async_graphql::Object;
use log::error;
use sqlx::PgPool;
use crate::utils;

#[Object(extends)]
impl PostQuery {
    async fn latest_posts<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Post>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_posts: FieldResult<Vec<PostModel>> = PostModel::get_latest_posts(&pool).await;
        match r_posts {
            Ok(posts) => Ok(PostModel::convert_all_to_gql(&posts)),
            Err(error) => {
                error!(
                            "Posts couldn't be fetched from the db due to error [{}]",
                            error.message
                        );
                Err(async_graphql::Error::new(
                    "Posts not found, error encountered",
                ))
            }
        }
    }

    async fn posts_for_user<'a>(
        &self,
        ctx: &'a Context<'_>,
        user_id: String,
    ) -> FieldResult<Vec<Post>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_posts: FieldResult<Vec<PostModel>> =
            PostModel::find_posts_for_user(&pool, &user_id).await;
        match r_posts {
            Ok(posts) => Ok(PostModel::convert_all_to_gql(&posts)),
            Err(error) => {
                error!(
                            "Posts for user with id [{}] not found due to error [{}]",
                            user_id, error.message
                        );
                Err(async_graphql::Error::new(
                    "Posts not found, error encountered",
                ))
            }
        }
    }

    async fn post_for_id<'a>(&self, ctx: &'a Context<'_>, post_id: String) -> FieldResult<Post> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let r_post: FieldResult<PostModel> =
            PostModel::find_post_for_id(&pool, &post_id).await;
        match r_post {
            Ok(post) => Ok(PostModel::convert_to_gql(&post)),
            Err(error) => {
                error!(
                            "Post for id [{}] not found due to error [{}]",
                            post_id, error.message
                        );
                Err(async_graphql::Error::new(
                    "Posts not found, error encountered",
                ))
            }
        }
    }
}
