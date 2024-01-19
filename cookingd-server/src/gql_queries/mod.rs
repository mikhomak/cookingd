use async_graphql::MergedObject;

mod user_query;
mod post_query;
mod tag_query;

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct PostQuery;

#[derive(Default)]
pub struct TagQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, PostQuery, TagQuery);
