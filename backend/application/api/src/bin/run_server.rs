use api::controller;
use dotenv;
use mysql::Pool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match Pool::new(url.as_str()) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to connect to tha database: {}", err);
            std::process::exit(1);
        }
    };

    let api = controller::Api { db: pool };
    let router =
        generated::server::new(api).layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, router).await.unwrap();
}
