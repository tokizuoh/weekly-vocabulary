use api::controller;
use dotenv;
use mysql::Pool;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match Pool::new(url.as_str()) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to connect to tha database: {}", err);
            std::process::exit(1);
        }
    };

    let api = controller::Api { db: pool };
    let router = generated::server::new(api);
    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, router).await.unwrap();
}
