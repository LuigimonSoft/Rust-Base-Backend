
use dotenv::dotenv;
use std::sync::Arc;

mod config;
mod repositories;
mod models;
mod services;
mod controllers;
mod errors;
mod middleware;



#[tokio::main]
async fn main(){
    dotenv().ok();

    let config = Arc::new(config::Config::from_env());
    let routes = controllers::base_routes(Arc::clone(&config));

    println!("Server starting on port {}", config.port);
    println!("API base path: /{}", config.api_base);
    warp::serve(routes)
        .run(([127, 0, 0, 1], config.port))
        .await;
}