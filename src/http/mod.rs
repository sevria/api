use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::get,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::Config,
    constant,
    context::Context,
    domain::{
        auth::{self, http::AuthState},
        data::{self, http::DataState},
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

pub fn new_router(config: Arc<Config>, context: Context) -> Router {
    let mut cors = CorsLayer::new()
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]);

    if config.cors_allow_origin == "*" {
        cors = cors.allow_origin(Any);
    } else {
        let origins: Vec<HeaderValue> = config
            .cors_allow_origin
            .split(",")
            .filter_map(|origin| HeaderValue::from_str(origin.trim()).ok())
            .collect();
        cors = cors.allow_origin(origins);
    }

    let auth_state = Arc::new(AuthState::new(context.auth_service));
    let data_state = Arc::new(DataState::new(context.data_service));
    let field_state = Arc::new(FieldState::new(context.field_service));
    let schema_state = Arc::new(SchemaState::new(context.schema_service));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/", get(health_check))
        .nest("/auth", auth::http::router(auth_state))
        .nest("/schemas", schema::http::router(schema_state))
        .nest(
            "/schemas/{schema_id}/fields",
            field::http::router(field_state),
        )
        .nest(
            "/schemas/{schema_name}/data",
            data::http::router(data_state),
        )
        .layer(cors)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/docs").url("/openapi.json", api));

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
