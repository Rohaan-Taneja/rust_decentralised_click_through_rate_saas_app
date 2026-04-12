use std::sync::Arc;

use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{AppState, routes::{task_routes::task_handler, user_routes::user_handler}};

pub fn create_route(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/user", user_handler())
        .nest("/task", task_handler())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api/v1", api_route)
}
