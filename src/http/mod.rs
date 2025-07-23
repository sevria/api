use std::sync::Arc;

use axum::{Router, routing::get};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{constant, context::Context, domain::schema, util::http::Json};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = constant::TAG_SCHEMA)
    )
)]
struct ApiDoc;

pub fn new_router(ctx: Arc<Context>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(health_check))
        .nest("/schemas", schema::http::router(ctx.clone()))
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
