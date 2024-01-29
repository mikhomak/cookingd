use async_graphql::{Context, FieldResult, InputObject, Upload};
use log::error;
use sqlx::PgPool;
use uuid::Uuid;
use crate::gql_models::post_gql_model::Post;
use crate::servies::site_configuration_service::is_posting_allowed;
use crate::psql_models::post_psql_model::PostModel;
use crate::gql_mutations::PostMutations;
use crate::psql_models::tag_psql_model::TagModel;
use crate::guards::role::RoleGuard;
use crate::guards::role::Role;
#[derive(InputObject)]
pub struct PostCreationInput {
    pub user_id: String,
    pub title: String,
    pub text: String,
    pub rating: Option<f64>,
    pub tags: Option<Vec<String>>,
    pub main_image: Option<Upload>,
    pub other_images: Option<Vec<Upload>>
}

#[derive(InputObject)]
pub struct TagAssignationInput {
    pub post_id: String,
    pub tag_names: Vec<String>,
}

#[async_graphql::Object]
impl PostMutations {

    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        post_input: PostCreationInput,
    ) -> FieldResult<Post> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let is_registration_enabled: bool = is_posting_allowed(pool).await;
                if is_registration_enabled == false {
                    return Err(async_graphql::Error::new("Posting failed!"));
                }

                let r_created_post : FieldResult<PostModel>= PostModel::create(&pool, &post_input).await;

                match r_created_post {
                    Ok(created_post) => {
                        if let Some(tags) = post_input.tags {
                            let _ = create_and_associate_tags_with_post_uuid(&pool, &created_post.id, &tags).await;
                        }
                        Ok(PostModel::convert_to_gql(&created_post))
                    }
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


    async fn assign_tags(
        &self,
        ctx: &Context<'_>,
        tag_input: TagAssignationInput,
    ) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                create_and_associate_tags(&pool, &tag_input.post_id, &tag_input.tag_names).await?;
                Ok(true)
            }
            Err(_) => {
                error!("Error at associating tags. Database is not set in context!");
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }


    async fn remove_tags(
        &self,
        ctx: &Context<'_>,
        tag_input: TagAssignationInput,
    ) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        match r_pool {
            Ok(pool) => {
                let post_uuid : sqlx::types::Uuid= sqlx::types::Uuid::parse_str(&tag_input.post_id).unwrap();
                TagModel::remove_tag_association(pool, &tag_input.tag_names, &post_uuid).await;
                Ok(true)
            }
            Err(_) => {
                error!("Error at associating tags. Database is not set in context!");
                Err(async_graphql::Error::new("Server error!"))
            }
        }
    }
}

async fn create_and_associate_tags(pool: &PgPool, post_id: &String, tags: &Vec<String>) -> Result<(), async_graphql::Error> {
    let r_post_uuid : Result<sqlx::types::Uuid, _> = Uuid::parse_str(post_id);
    match r_post_uuid {
        Ok(post_uuid) => {
            create_and_associate_tags_with_post_uuid(&pool, &post_uuid, &tags).await
        }
        Err(_) => {
            error!("Cannot parse post uuid!");
            Err(async_graphql::Error::new("Couldn't create tags!"))
        }
    }
}

async fn create_and_associate_tags_with_post_uuid(pool: &PgPool, post_uuid: &Uuid, tags: &Vec<String>) -> Result<(), async_graphql::Error> {
    let r_created_tags = TagModel::create_batch_tags(
        &pool,
        &tags).await;
    match r_created_tags {
        Ok(created_tags) => {
            TagModel::associate_tags_to_post(&pool, &created_tags, &post_uuid).await;
            Ok(())
        }
        Err(error) => {
            error!("Error while creating tags! Error - {0}", error.message);
            Err(async_graphql::Error::new("Couldn't create tags!"))
        }
    }
}