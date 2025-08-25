use crate::config;
use crate::controllers;
use crate::errors;
use crate::swagger;
use crate::swagger::serve_swagger;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::oneshot;

use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::{Filter, Rejection};

pub async fn run_server() -> (oneshot::Sender<()>, String) {
    dotenv().ok();

    let config = Arc::new(config::Config::from_env());
    let config_swagger = Arc::new(Config::from(format!("/{}/api-doc.json", config.api_base)));
    let routes = controllers::routes(Arc::clone(&config));
    let static_files = warp::fs::dir(config.static_dir.clone());

    let port: u16 = config.port;
    let api_base = config.api_base.trim_matches('/').to_string();
    let api_segments: Vec<String> = api_base.split('/').map(|s| s.to_string()).collect();

    let mut api_path = warp::path(api_segments[0].clone()).boxed();
    for segment in &api_segments[1..] {
        api_path = api_path.and(warp::path(segment.clone())).boxed();
    }

    let api_doc = api_path
        .clone()
        .and(warp::path("api-doc.json"))
        .and(warp::get())
        .and_then(|| async { Ok::<_, Rejection>(warp::reply::json(&swagger::ApiDoc::openapi())) });

    let swagger_ui = api_path
        .clone()
        .and(warp::path("swagger-ui"))
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config_swagger.clone()))
        .and_then(serve_swagger);

    let (tx, rx) = oneshot::channel();
    let routes = api_doc
        .or(swagger_ui)
        .or(routes)
        .or(static_files)
        .recover(errors::handle_rejection);

    let socket_addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("failed to bind address");
    let addr = listener.local_addr().expect("failed to get local addr");

    let server = warp::serve(routes)
        .incoming(listener)
        .graceful(async {
            rx.await.ok();
        });

    tokio::spawn(server.run());

    println!("Server starting on port {}", addr.port());
    println!("API base path: /{}", config.api_base);

    (tx, format!("http://127.0.0.1:{}", addr.port()))
}
