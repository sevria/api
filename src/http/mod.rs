use axum::Router;

use crate::domains::schemas;

pub fn new_router() -> Router {
    Router::new().nest("/schemas/", schemas::http::routes())
}
