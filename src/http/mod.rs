use std::sync::Arc;

use axum::{Router, routing::get};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    constant,
    context::Context,
    domain::{
        auth::{self, http::AuthState},
        field::{self, http::FieldState},
        schema::{self, http::SchemaState},
    },
    util::http::Json,
};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = constant::TAG_SCHEMA)
    )
)]
struct ApiDoc;

pub fn new_router(context: Context) -> Router {
    let schema_state = Arc::new(SchemaState::new(context.schema_service));
    let field_state = Arc::new(FieldState::new(context.field_service));
    let auth_state = Arc::new(AuthState::new(context.auth_service));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(health_check))
        .nest("/auth", auth::http::router(auth_state))
        .nest("/schemas", schema::http::router(schema_state))
        .nest(
            "/schemas/{schema_id}/fields",
            field::http::router(field_state),
        )
        .split_for_parts();

    let router = router.merge(Scalar::with_url("/docs", api));

    router
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: String::from("ok"),
    })
}
