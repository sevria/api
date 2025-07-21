use axum::{Json, extract::Path};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    domain::schema::model::{CreateSchemaRequest, Schema, UpdateSchemaRequest},
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_schema, get_schemas,))
        .routes(routes!(get_schema, update_schema, delete_schema))
}

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
async fn create_schema(Json(body): Json<CreateSchemaRequest>) -> Json<Schema> {
    Json(Schema {
        id: body.name.clone(),
        name: body.name,
        fields: body.fields,
    })
}

#[utoipa::path(
    get,
    path = "",
    summary = "Get schemas",
    tag = constant::TAG_SCHEMA,
    responses(
        (status = 200, body = Vec<Schema>)
    )
)]
async fn get_schemas() -> Json<Vec<Schema>> {
    Json(vec![])
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
