#![allow(unused)]
use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]

async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async {Html("Hello <strong>World!</strong>")})
    );

    // region: --start Server
    // let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello)
    .await
    .unwrap();
        // endregion --start Server
    println!("End server");
}

