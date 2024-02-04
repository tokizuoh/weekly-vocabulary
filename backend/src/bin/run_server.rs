use weekly_vocabulary::route::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();
    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, app).await.unwrap();
}
