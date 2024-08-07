
use dotenv::dotenv;
use std::sync::Arc;

use std::net::Ipv4Addr;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, openapi::Server
};
use utoipa_swagger_ui::Config;
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};
use crate::swagger::serve_swagger;

mod config;
mod swagger;
mod repositories;
mod models;
mod services;
mod controllers;
mod errors;
mod middleware;
mod validators;


#[tokio::main]
async fn main(){
    dotenv().ok();

    let config = Arc::new(config::Config::from_env());
    let config_swagger = Arc::new(Config::from(format!("/{}/api-doc.json", config.api_base)));
    let routes = controllers::base_routes(Arc::clone(&config));

    let api_base = config.api_base.trim_matches('/').to_string();
    let api_segments: Vec<String> = api_base.split('/').map(|s| s.to_string()).collect();

    let mut api_path = warp::path(api_segments[0].clone()).boxed();
    for segment in &api_segments[1..] {
        api_path = api_path.and(warp::path(segment.clone())).boxed();
    }

    let api_doc = api_path.clone()
        .and(warp::path("api-doc.json"))
        .and(warp::get())
        .map(|| warp::reply::json(&swagger::ApiDoc::openapi()));

    let swagger_ui = api_path.clone()
        .and(warp::path("swagger-ui"))
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config_swagger.clone()))
        .and_then(serve_swagger);

    println!("Server starting on port {}", config.port);
    println!("API base path: /{}", config.api_base);
    warp::serve(api_doc.or(swagger_ui).or(routes))
        .run(([127, 0, 0, 1], config.port))
        .await;
}