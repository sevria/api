use std::sync::Arc;

use axum::{Router, routing::get};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    constant,
    domain::{
        field::{
            self, http::FieldState, repository_impl::FieldRepositoryImpl, service::FieldService,
        },
        schema::{
            self, http::SchemaState, repository_impl::SchemaRepositoryImpl, service::SchemaService,
        },
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

pub fn new_router(db: Arc<Pool<Postgres>>) -> Router {
    let schema_repository = Arc::new(SchemaRepositoryImpl::new(db.clone()));
    let field_repository = Arc::new(FieldRepositoryImpl::new(db.clone()));

    let schema_service = Arc::new(SchemaService::new(schema_repository));
    let field_service = Arc::new(FieldService::new(field_repository));

    let schema_state = Arc::new(SchemaState::new(schema_service));
    let field_state = Arc::new(FieldState::new(field_service));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(health_check))
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
