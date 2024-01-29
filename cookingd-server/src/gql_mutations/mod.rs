use async_graphql::MergedObject;

pub mod user_mutation;
pub mod post_mutation;
pub mod login_mutation;

#[derive(Default)]
pub struct UserMutations;
#[derive(Default)]
pub struct PostMutations;
#[derive(Default)]
pub struct LoginMutations;

#[derive(MergedObject, Default)]
pub struct Mutations(UserMutations, PostMutations, LoginMutations);
