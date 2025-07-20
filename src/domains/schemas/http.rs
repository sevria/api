use axum::{
    Json, Router,
    routing::{delete, get, patch, post},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_schema))
        .route("/", get(get_schemas))
        .route("/{id}", get(get_schema))
        .route("/{id}", patch(update_schema))
        .route("/{id}", delete(delete_schema))
}

async fn create_schema() -> Json<String> {
    Json(String::from("todo: create_schema"))
}

async fn get_schemas() -> Json<String> {
    Json(String::from("todo: get_schemas"))
}

async fn get_schema() -> Json<String> {
    Json(String::from("todo: get_schema"))
}

async fn update_schema() -> Json<String> {
    Json(String::from("todo: update_schema"))
}

async fn delete_schema() -> Json<String> {
    Json(String::from("todo: delete_schema"))
}
