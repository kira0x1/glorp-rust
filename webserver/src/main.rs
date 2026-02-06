mod templater;

use askama::Template;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let hello = templater::HelloTemplate { name: "meow" };
    let app = Router::new().route("/", get(hello.render().unwrap()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
