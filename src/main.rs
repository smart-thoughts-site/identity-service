use axum::{
    routing::{get,post},
    Router
};
use std::net::SocketAddr;

mod authentication;
mod persistence;
mod rest;

#[tokio::main]
async fn main() {
    persistence::init().await;

    let state = authentication::new();
            
    let app = Router::new()
        .route("/", get(rest::index))
        .route("/login", post(rest::login))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
