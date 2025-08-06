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

pub struct SchemaState {
    pub schema_service: Arc<SchemaService>,
}

impl SchemaState {
    pub fn new(schema_service: Arc<SchemaService>) -> SchemaState {
        SchemaState { schema_service }
    }
}

pub fn router(state: Arc<SchemaState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create, list))
        .routes(routes!(get, update, delete))
        .with_state(state)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "",
    operation_id = "create_schema",
    summary = "Create schema",
    tag = constant::TAG_SCHEMA,
    request_body = CreateSchemaRequest,
    responses(
        (status = 200, body = Schema)
    )
)]
async fn create(
    State(state): State<Arc<SchemaState>>,
    Json(req): Json<CreateSchemaRequest>,
) -> Result<Json<Schema>, Error> {
    let schema = state.schema_service.create(&req).await?;
    Ok(Json(schema))
}

#[utoipa::path(
    get,
    path = "",
    operation_id = "list_schemas",
    summary = "List schemas",
    tag = constant::TAG_SCHEMA,
    responses(
        (status = 200, body = Paginated<Schema>)
    )
)]
async fn list(State(state): State<Arc<SchemaState>>) -> Result<Json<Paginated<Schema>>, Error> {
    Ok(Json(state.schema_service.list().await?))
}

#[utoipa::path(
    get,
    path = "/{id}",
    operation_id = "get_schema",
    summary = "Get schema",
    tag = constant::TAG_SCHEMA,
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 200, body = Schema)
    )
)]
async fn get(
    State(state): State<Arc<SchemaState>>,
    Path(id): Path<String>,
) -> Result<Json<Schema>, Error> {
    Ok(Json(state.schema_service.get(&id).await?))
}

#[utoipa::path(
    patch,
    path = "/{id}",
    operation_id = "update_schema",
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
async fn update(Path(id): Path<String>, Json(req): Json<UpdateSchemaRequest>) -> Json<Schema> {
    Json(Schema {
        id,
        name: req.name.unwrap_or_default(),
        fields: vec![],
    })
}

#[utoipa::path(
    delete,
    path = "/{id}",
    operation_id = "delete_schema",
    summary = "Delete schema",
    tag = constant::TAG_SCHEMA,
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 200, body = Schema)
    )
)]
async fn delete(Path(id): Path<String>) -> Json<Schema> {
    Json(Schema {
        id,
        name: "".to_string(),
        fields: vec![],
    })
}
