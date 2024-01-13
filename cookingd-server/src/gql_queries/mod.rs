use async_graphql::MergedObject;

mod user_query;
mod post_query;

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct PostQuery;
#[derive(MergedObject, Default)]
pub struct Query(UserQuery, PostQuery);
