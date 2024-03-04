use actix_web::http::header::HeaderMap;
use actix_web::{HttpRequest, web};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::CookingSchema;
use crate::guards::role::Role;

pub struct Token(pub String);

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header,  Validation, EncodingKey, DecodingKey, TokenData};
use jsonwebtoken::errors::Error;
use log::{error};

#[derive(Debug, Serialize, Deserialize)]
pub struct CookingdClaims {
    pub id: String,
    pub email: String,
    exp: usize,
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

pub async fn index_token(
    schema: web::Data<CookingSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    if let Some(token) = get_token_from_headers(req.headers()) {
        let r_token = get_token(&token.0);
        match r_token {
            Ok(jsonwebtoken) => {
                request = request.data(jsonwebtoken.claims);
                request = request.data(Role::User);
            }
            Err(error) => {
                error!("Cannot decode a token for the user with token [{}] due to error [{}]", &token.0, error);
                request = request.data(Role::Anon);
            }
        };

    } else {
        request = request.data(Role::Anon);
    }

    schema.execute(request).await.into()
}


pub fn create_token(id: &String, email: &String) -> Result<String, Error> {
    let my_claims = CookingdClaims {
        id: id.clone(),
        email: email.clone(),
        exp: 100000000000000,
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(env!("AUTH_SECRET").as_ref()))?;
    Ok(token)
}

pub fn get_token(token: &String) -> Result<TokenData<CookingdClaims>, Error> {
    let token = decode::<CookingdClaims>(token, &DecodingKey::from_secret(env!("AUTH_SECRET").as_ref()), &Validation::default())?;
    Ok(token)
}