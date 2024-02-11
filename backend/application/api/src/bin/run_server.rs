use api::{app_state::AppState, route::create_router};
use dotenv;
use mysql::Pool;
use std::sync::Arc;

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

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = create_router(app_state);
    let lister = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(lister, app).await.unwrap();
}
