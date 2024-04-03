use actix_cors::Cors;
use actix_files;
use actix_web::{guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::PgPool;
use std::env;
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

        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(web::resource("/").guard(guard::Post()).to(index_token))
            .service(
                actix_files::Files::new("/images", f_image_dir.clone())
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/playground")
                    .guard(guard::Get())
                    .to(index_playground),
            )
            .route("/ping", web::get().to(ping))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
