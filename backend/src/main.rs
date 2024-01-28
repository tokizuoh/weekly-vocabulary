use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/get/recent",
            get(|| weekly_vocabulary::vocabulary::get_latest_vocabulary()),
        )
        .route("/get/all", post("TODO"))
        .route("/register", post("TODO"))
    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, app).await.unwrap();
}
