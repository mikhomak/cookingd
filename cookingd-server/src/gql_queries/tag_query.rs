use crate::gql_models::tag_gql_model::Tag;
use crate::gql_queries::TagQuery;
use crate::guards::role::Role;
use crate::guards::role::RoleGuard;
use crate::psql_models::tag_psql_model::TagModel;
use async_graphql::Context;
use async_graphql::FieldResult;
use async_graphql::Object;
use log::error;
use sqlx::PgPool;

#[Object(extends)]
impl TagQuery {
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn all_tags<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<Tag>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_tag_models: FieldResult<Vec<TagModel>> = TagModel::get_all(pool).await;
                match r_tag_models {
                    Ok(tag_models) => Ok(TagModel::convert_all_to_gql(&tag_models)),
                    Err(error) => {
                        error!(
                            "There was an error fetching tags, error is {}",
                            error.message
                        );
                        Err(async_graphql::Error::new("Cannot fetch tags!"))
                    }
                }
            }
            Err(_) => {
                error!("Database is not set up in the context");
                Err(async_graphql::Error::new("Server Error!"))
            }
        }
    }

    async fn tag_by_name<'a>(&self, ctx: &'a Context<'_>, name: String) -> FieldResult<Vec<Tag>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_tag_models: FieldResult<Vec<TagModel>> =
                    TagModel::find_tags_for_name(pool, &name).await;
                match r_tag_models {
                    Ok(tag_models) => Ok(TagModel::convert_all_to_gql(&tag_models)),
                    Err(error) => {
                        error!(
                            "There was an error fetching tags with name {}, error is {}",
                            name, error.message
                        );
                        Err(async_graphql::Error::new("Cannot fetch tags!"))
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
