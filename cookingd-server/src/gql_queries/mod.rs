use async_graphql::MergedObject;

mod login_query;
mod post_query;
mod tag_query;
mod user_query;

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct PostQuery;

#[derive(Default)]
pub struct TagQuery;

#[derive(Default)]
pub struct LoginQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, PostQuery, TagQuery, LoginQuery);
