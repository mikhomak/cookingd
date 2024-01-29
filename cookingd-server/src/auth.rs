use actix_web::http::header::HeaderMap;
use actix_web::{HttpRequest, web};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::CookingSchema;
use crate::guards::role::Role;

pub struct Token(pub String);

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("login")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

pub async fn index_token(
    schema: web::Data<CookingSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(Role::User);
    }else {
        request = request.data(Role::Anon);
    }

    schema.execute(request).await.into()
}