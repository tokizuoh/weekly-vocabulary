use crate::{
    app_state::AppState,
    vocabulary::{
        delete_vocabulary, get_all_vocabulary, get_latest_vocabulary, register_vocabulary,
    },
};
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get/recent", get(get_latest_vocabulary))
        .route("/get/all", get(get_all_vocabulary))
        .route("/register", post(register_vocabulary))
        .route("/update", delete("TODO"))
        .route("/delete", delete(delete_vocabulary))
        .with_state(app_state)
}
