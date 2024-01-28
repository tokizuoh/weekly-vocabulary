use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/get/vocabulary/recent",
            get(|| weekly_vocabulary::vocabulary::get_latest_vocabulary()),
        )
        .route("/get/vocabulary/all", post("TODO"))
        .route("/register/vocaburary", post("TODO"));
    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, app).await.unwrap();
}
