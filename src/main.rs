use std::net::SocketAddr;

use axum::response::Html;
use axum::{routing::method_routing, Router};
#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/",
        method_routing::get(|| async { Html("<body style = \"background-color:black;\"><h1 style=\"color:white;\"> hello Axum</h1></body>") }),
    );
    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}
