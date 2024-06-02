use actix_cors::Cors;
use actix_files;
use actix_web::{guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, error};
use anyhow::Result;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::PgPool;
use std::env;
use actix_web::dev::{Service, ServiceRequest};
use web::Data;

use crate::auth::index_token;
use crate::gql_mutations::Mutations;
use crate::gql_queries::Query;

mod auth;
mod gql_models;
mod gql_mutations;
mod gql_queries;
mod guards;
mod psql_models;
mod services;
mod utils;

pub type CookingSchema = Schema<Query, Mutations, EmptySubscription>;

async fn index(schema: Data<CookingSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn ping(_req: HttpRequest) -> impl Responder {
    format!(
        "I am healthy: {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    )
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let env: String = env::var("ENV").unwrap_or("local".to_string());

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let host: String = env::var("HOST").expect("HOST is not set");
    let port: String = env::var("PORT").expect("PORT is not set");
    let db_pool: PgPool = PgPool::connect(&database_url).await?;
    sqlx::migrate!().run(&db_pool).await?;

    let schema: CookingSchema =
        Schema::build(Query::default(), Mutations::default(), EmptySubscription)
            .data(db_pool)
            .finish();

    env_logger::init();
    let f_image_dir: String = dotenv::var("IMAGES_DIR").unwrap_or("images/".to_string());

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(env!("FRONT_URL"))
            .allowed_origin(env!("FRONT_URL_2"))
            .allowed_origin(env!("FRONT_URL_3"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION.to_string(),
                http::header::ACCEPT.to_string(),
            ])
            .allowed_header(http::header::CONTENT_TYPE.to_string())
            .max_age(3600);

        let playground_cookie: &str = env!("PLAYGROUND_COOKIE");
        let playground_cookie_value: &str = env!("PLAYGROUND_COOKIE_VALUE");
        let is_local: bool = env.eq("local");

        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(web::resource("/api").guard(guard::Post()).to(index_token))
            .service(
                actix_files::Files::new("/api/images", f_image_dir.clone())
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/api/playground")
                    .wrap_fn(move |http_request: ServiceRequest, srv| {
                        let has_cookie: bool = http_request.cookie(playground_cookie)
                            .map(|cookie| cookie.value().to_string())
                            .map(|value| value.eq(playground_cookie_value))
                            .unwrap_or(false)
                            .clone();
                        let fut = srv.call(http_request);
                        async move {
                            if !is_local && !has_cookie {
                                return Err(error::ErrorInternalServerError(""));
                            }
                            Ok(fut.await?)
                        }
                    })
                    .guard(guard::Get())
                    .to(index_playground),
            )
            .route("/api/ping", web::get().to(ping))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
