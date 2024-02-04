use crate::{app_state::AppState, vocabulary::get_latest_vocabulary};
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get/recent", get(get_latest_vocabulary))
        .route("/get/all", post("TODO"))
        .route("/register", post("TODO"))
        .route("/update", delete("TODO"))
        .route("/delete", delete("TODO"))
        .with_state(app_state)
}
