use crate::auth::CookingdClaims;
use crate::gql_models::post_gql_model::Post;
use crate::gql_mutations::PostMutations;
use crate::guards::role::Role;
use crate::guards::role::RoleGuard;
use crate::psql_models::post_psql_model::PostModel;
use crate::psql_models::tag_psql_model::TagModel;
use crate::services::image_service;
use crate::services::site_configuration_service::is_posting_allowed;
use crate::utils;
use anyhow::Error;
use async_graphql::{Context, FieldResult, InputObject, Upload};
use log::{error, info};
use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[derive(InputObject)]
pub struct PostCreationInput {
    pub title: String,
    pub text: String,
    pub rating: Option<f64>,
    pub tags: Option<Vec<String>>,
    pub main_image: Option<Upload>,
    pub other_images: Option<Vec<Upload>>,
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

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let is_registration_enabled: bool = is_posting_allowed(pool).await;
        if is_registration_enabled == false {
            return Err(async_graphql::Error::new("Posting failed!"));
        }

        let r_user = ctx.data::<CookingdClaims>();

        let user_claims = r_user.map_err(|_| {
            error!("Cannot create a post due to the user not being present in the context");
            return Err::<CookingdClaims, async_graphql::Error>(async_graphql::Error::new("Posting failed! Bad user"));
        }).unwrap();

        let r_created_post: FieldResult<PostModel> =
            PostModel::create(&pool, ctx, &post_input, &user_claims.id).await;

        match r_created_post {
            Ok(created_post) => {
                if let Some(ref tags) = post_input.tags {
                    let _ = create_and_associate_tags_with_post_uuid(
                        &pool,
                        &created_post.id,
                        &tags,
                        Some(&user_claims.id),
                    ).await;
                }

                store_image(
                    post_input,
                    &created_post.id.to_string(),
                    &user_claims.id,
                    ctx,
                ).await?;

                Ok(PostModel::convert_to_gql(&created_post))
            }
            Err(error) => {
                error!("Cannot create a post due to error [{}]", error.message);
                Err(async_graphql::Error::new("Posting failed!"))
            }
        }
    }


    async fn assign_tags(
        &self,
        ctx: &Context<'_>,
        tag_input: TagAssignationInput,
    ) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        create_and_associate_tags(&pool, &tag_input.post_id, &tag_input.tag_names, None)
            .await?;
        Ok(true)
    }

    async fn remove_tags(
        &self,
        ctx: &Context<'_>,
        tag_input: TagAssignationInput,
    ) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();

        let Ok(pool) = r_pool else {
            return Err(utils::error_database_not_setup());
        };

        let post_uuid: sqlx::types::Uuid =
            sqlx::types::Uuid::parse_str(&tag_input.post_id).unwrap();
        TagModel::remove_tag_association(pool, &tag_input.tag_names, &post_uuid).await;
        Ok(true)
    }

    async fn delete_post(&self, ctx: &Context<'_>, post_id: String) -> FieldResult<bool> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        let pool = r_pool.map_err(|_| { return Err::<&PgPool, async_graphql::Error>(utils::error_database_not_setup()); }).unwrap();


        match PostModel::delete_post_for_id(pool, &post_id).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

async fn create_and_associate_tags(
    pool: &PgPool,
    post_id: &String,
    tags: &Vec<String>,
    user_id: Option<&str>,
) -> Result<(), async_graphql::Error> {
    let r_post_uuid: Result<Uuid, _> = Uuid::parse_str(post_id);
    match r_post_uuid {
        Ok(post_uuid) => {
            create_and_associate_tags_with_post_uuid(&pool, &post_uuid, &tags, user_id).await
        }
        Err(_) => {
            error!("Cannot parse post uuid!");
            Err(async_graphql::Error::new("Couldn't create tags!"))
        }
    }
}

async fn create_and_associate_tags_with_post_uuid(
    pool: &PgPool,
    post_uuid: &Uuid,
    tags: &Vec<String>,
    user_id: Option<&str>,
) -> Result<(), async_graphql::Error> {
    let r_created_tags = TagModel::create_batch_tags(&pool, &tags, user_id).await;
    match r_created_tags {
        Ok(created_tags) => {
            TagModel::associate_tags_to_post(&pool, &created_tags, &post_uuid).await;
            Ok(())
        }
        Err(error) => {
            error!("Error while creating tags! Error - [{0}]", error.message);
            Err(async_graphql::Error::new("Couldn't create tags!"))
        }
    }
}

async fn store_image(
    post_input: PostCreationInput,
    post_uid_as_str: &str,
    user_guid_as_str: &str,
    ctx: &Context<'_>,
) -> Result<(), Error> {
    if let Some(main_image) = post_input.main_image {
        if let Ok(main_image_value) = main_image.value(ctx) {
            let image_type: &str = &*main_image_value.content_type.unwrap();

            let mapped_image_type = image_service::map_image_type(&image_type);

            let f_image_dir: String = dotenv::var("IMAGES_DIR").unwrap_or("images/".to_string());

            let image_user_dir: String =
                image_service::construct_image_user_dir(&*post_uid_as_str, user_guid_as_str)?;
            let image_name: String =
                image_service::construct_image_title(mapped_image_type.unwrap().as_str())?;

            info!("trying to create an image in [{0}]", f_image_dir);

            let dir: String = format!("{}/{}", f_image_dir, image_user_dir);
            info!("trying to create a directory [{0}]", dir.clone());
            match tokio::fs::create_dir_all(dir.clone()).await {
                Ok(_) => {
                    match File::create(dir.to_string() + &*image_name).await {
                        Ok(mut created_file) => {
                            match created_file.write_all(&main_image_value.content).await {
                                Ok(_) => {}
                                Err(error) => {
                                    error!(
                                        "Error while writing to the file, error - [{0}]",
                                        error.to_string()
                                    )
                                }
                            };
                        }
                        Err(error) => {
                            error!(
                                "Error while creating file, error - [{0}]",
                                error.to_string()
                            )
                        }
                    };
                }
                Err(error) => {
                    error!(
                        "Error while creating an dir, error -  [{0}]",
                        error.to_string()
                    )
                }
            };
        }
    }
    Ok(())
}
