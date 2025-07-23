use std::sync::Arc;

use axum::extract::{Path, State};
use nanoid::nanoid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    context::Context,
    domain::schema::model::{CreateSchemaRequest, Schema, UpdateSchemaRequest},
    util::{
        error::{self, Error},
        http::Json,
    },
};

pub fn router(ctx: Arc<Context>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_schema, get_schemas))
        .routes(routes!(get_schema, update_schema, delete_schema))
        .with_state(ctx)
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
    State(ctx): State<Arc<Context>>,
    Json(req): Json<CreateSchemaRequest>,
) -> Result<Json<Schema>, Error> {
    let mut tx = ctx.db.begin().await.unwrap();

    let mut schema = Schema {
        id: nanoid!(),
        name: req.name,
        fields: vec![],
    };

    // Insert schema
    let _: Result<(), Error> = match sqlx::query!(
        "INSERT INTO schemas (id, name) VALUES ($1, $2)",
        schema.id,
        schema.name
    )
    .execute(&mut *tx)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("failed to insert schema: {}", err);
            return Err(error::internal());
        }
    };

    // Insert each field
    for field in req.fields {
        let id = nanoid!();

        let _: Result<(), Error> = match sqlx::query!(
            "INSERT INTO schema_fields (id, schema_id, name, value_type, required, default_value)
             VALUES ($1, $2, $3, $4, $5, $6)",
            id,
            schema.id,
            field.name,
            field.value_type,
            field.required,
            field.default_value
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("failed to insert schema field: {}", err);
                return Err(error::internal());
            }
        };

        schema.fields.push(field);
    }

    let _: Result<(), Error> = match tx.commit().await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("failed to commit transaction: {}", err);
            return Err(error::internal());
        }
    };

    Ok(Json(schema))
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
