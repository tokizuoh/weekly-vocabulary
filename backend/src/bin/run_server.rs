use axum::{
    routing::{delete, get, post},
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
        .route("/update", delete("TODO"))
        .route("/delete", delete("TODO"));
    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, app).await.unwrap();
}
