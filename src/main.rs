
use crate::server::run_server;

mod config;
mod swagger;
mod repositories;
mod models;
mod services;
mod controllers;
mod errors;
mod middleware;
mod validators;
mod router;
mod server;




#[tokio::main]
async fn main(){
    
    let(tx,_) = run_server().await;

    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    println!("Shutting down server...");
    
    let _ = tx.send(());
}