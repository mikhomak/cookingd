use crate::gql_queries::Query;
use async_graphql::Object;
use async_graphql::Context;
use sqlx::PgPool;
use async_graphql::FieldResult;
use log::error;

use crate::gql_models::user_model::User;

#[Object(extends)]
impl Query {
    async fn users<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<Vec<User>> {
        let r_pool: Result<&PgPool, async_graphql::Error> = ctx.data::<PgPool>();
        match r_pool {
            Ok(pool) => {
                let r_users = User::read_all(&pool).await;
                match r_users {
                    Ok(users)=> Ok(users),
                    Err(error)=>{
                        error!("Users couldn't be fetched from the db due to error {}", error.to_string());
                        Err(async_graphql::Error::new("Users not found"))
                    }
                }
            }
            Err(_) => {
                error!("Database is not set up in the context");
                Err(async_graphql::Error::new("Server Error!"))
            }
        }
    }

    async fn user<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = User::read_one(&pool, &id).await?;
        Ok(row)
    }

    #[graphql(entity)]
    async fn find_user_by_id<'a>(&self, ctx: &'a Context<'_>, id: String) -> FieldResult<User> {
        let pool = ctx.data::<PgPool>().unwrap();
        let row = User::read_one(&pool, &id).await?;
        Ok(row)
    }
}