use axum::{
    routing::{get,post},
    Router
};
use std::net::SocketAddr;

mod auth;
mod runtime;

#[tokio::main]
async fn main() {
    runtime::db::init().await;

    let state = auth::passwords::new();
            
    let app = Router::new()
        .route("/", get(runtime::rest::index))
        .route("/login", post(auth::rest::login))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
