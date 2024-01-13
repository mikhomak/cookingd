use async_graphql::MergedObject;

pub mod user_mutation;
pub mod post_mutation;

#[derive(Default)]
pub struct UserMutations;
#[derive(Default)]
pub struct PostMutations;
#[derive(MergedObject, Default)]
pub struct Mutations(UserMutations, PostMutations);
