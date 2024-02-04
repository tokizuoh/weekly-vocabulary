use crate::vocabulary::get_latest_vocabulary;
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/get/recent", get(|| get_latest_vocabulary()))
        .route("/get/all", post("TODO"))
        .route("/register", post("TODO"))
        .route("/update", delete("TODO"))
        .route("/delete", delete("TODO"))
}
