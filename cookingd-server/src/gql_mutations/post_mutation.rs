use async_graphql::{Context, FieldResult, InputObject};
use log::error;
use sqlx::PgPool;
use crate::servies::site_configuration_service::is_posting_allowed;
use crate::gql_models::post_model::Post;
use crate::gql_mutations::PostMutations;
#[derive(InputObject)]
pub struct PostCreationInput {
    pub user_id: String,
    pub title: String,
    pub text: String,
    pub rating: Option<f64>,
}

#[async_graphql::Object]
impl PostMutations {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        post_input: PostCreationInput,
    ) -> FieldResult<Post> {
        let r_pool: anyhow::Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let is_registration_enabled : bool = is_posting_allowed(pool).await;
                if is_registration_enabled  == false {
                    return Err(async_graphql::Error::new("Posting failed!"));
                }

                let r_created_post = Post::create(&pool, &post_input).await;

                match r_created_post {
                    Ok(created_post) => Ok(created_post),
                    Err(_) => {
                        error!("Cannot create a post due to error");
                        Err(async_graphql::Error::new("Posting failed!"))
                    }
                }
            }
            Err(_) => {
                error!("Error at creating a user. Database is not set in context!");
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }
}