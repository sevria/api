use std::sync::Arc;

use axum::extract::{Path, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    domain::field::{
        model::{CreateFieldRequest, Field},
        service::FieldService,
    },
    util::{error::Error, http::Json, paginator::Paginated},
};

pub struct FieldState {
    pub field_service: Arc<FieldService>,
}

impl FieldState {
    pub fn new(field_service: Arc<FieldService>) -> FieldState {
        FieldState { field_service }
    }
}

pub fn router(state: Arc<FieldState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create, list))
        .with_state(state)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "",
    summary = "Create field",
    tag = constant::TAG_FIELD,
    params(
        ("schema_id" = String, Path)
    ),
    request_body = CreateFieldRequest,
    responses(
        (status = 200, body = Field)
    )
)]
async fn create(
    State(state): State<Arc<FieldState>>,
    Path(schema_id): Path<String>,
    Json(mut req): Json<CreateFieldRequest>,
) -> Result<Json<Field>, Error> {
    req.schema_id = schema_id;
    let field = state.field_service.create(&req).await?;
    Ok(Json(field))
}

#[utoipa::path(
    get,
    path = "",
    summary = "List fields",
    tag = constant::TAG_FIELD,
    params(
        ("schema_id" = String, Path)
    ),
    responses(
        (status = 200, body = Paginated<Field>)
    )
)]
async fn list(
    State(state): State<Arc<FieldState>>,
    Path(schema_id): Path<String>,
) -> Result<Json<Vec<Field>>, Error> {
    Ok(Json(state.field_service.list(schema_id).await?))
}
