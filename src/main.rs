use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| weekly_vocabulary::run()));
    let lister = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(lister, app).await.unwrap();
}
