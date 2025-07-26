use std::sync::Arc;

use axum::extract::{Path, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    domain::schema::{
        model::{CreateSchemaRequest, Schema, UpdateSchemaRequest},
        service::SchemaService,
    },
    util::{error::Error, http::Json, paginator::Paginated},
};

pub struct SchemaRouterState {
    pub schema_service: Arc<SchemaService>,
}

impl SchemaRouterState {
    pub fn new(schema_service: Arc<SchemaService>) -> SchemaRouterState {
        SchemaRouterState { schema_service }
    }
}

pub fn router(state: Arc<SchemaRouterState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_schema, get_schemas))
        .routes(routes!(get_schema, update_schema, delete_schema))
        .with_state(state)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "",
    summary = "Create schema",
    tag = constant::TAG_SCHEMA,
    request_body = CreateSchemaRequest,
    responses(
        (status = 200, body = Schema)
    )
)]
async fn create_schema(
    State(state): State<Arc<SchemaRouterState>>,
    Json(req): Json<CreateSchemaRequest>,
) -> Result<Json<Schema>, Error> {
    let schema = state.schema_service.create_schema(&req).await?;
    Ok(Json(schema))
}

#[utoipa::path(
    get,
    path = "",
    summary = "Get schemas",
    tag = constant::TAG_SCHEMA,
    responses(
        (status = 200, body = Paginated<Schema>)
    )
)]
async fn get_schemas(
    State(state): State<Arc<SchemaRouterState>>,
) -> Result<Json<Paginated<Schema>>, Error> {
    Ok(Json(state.schema_service.get_schemas().await?))
}

#[utoipa::path(
    get,
    path = "/{id}",
    summary = "Get schema",
    tag = constant::TAG_SCHEMA,
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 200, body = Schema)
    )
)]
async fn get_schema(Path(id): Path<String>) -> Json<Schema> {
    Json(Schema {
        id: id.clone(),
        name: format!("Name of {}", id),
        fields: vec![],
    })
}

#[utoipa::path(
    patch,
    path = "/{id}",
    summary = "Update schema",
    tag = constant::TAG_SCHEMA,
    params(
        ("id" = String, Path)
    ),
    request_body = UpdateSchemaRequest,
    responses(
        (status = 200, body = Schema)
    )
)]
async fn update_schema(
    Path(id): Path<String>,
    Json(req): Json<UpdateSchemaRequest>,
) -> Json<Schema> {
    Json(Schema {
        id,
        name: req.name.unwrap_or_default(),
        fields: req.fields.unwrap_or_default(),
    })
}

#[utoipa::path(
    delete,
    path = "/{id}",
    summary = "Delete schema",
    tag = constant::TAG_SCHEMA,
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 200, body = Schema)
    )
)]
async fn delete_schema(Path(id): Path<String>) -> Json<Schema> {
    Json(Schema {
        id: id.clone(),
        name: format!("Name of {}", id),
        fields: vec![],
    })
}
