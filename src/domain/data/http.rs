use std::sync::Arc;

use axum::extract::{Path, State};
use serde_json::Value;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    util::{error::Error, http::Json},
};

use super::{model::CreateDataRequest, service::DataService};

pub struct DataState {
    pub data_service: Arc<DataService>,
}

impl DataState {
    pub fn new(data_service: Arc<DataService>) -> DataState {
        DataState { data_service }
    }
}

pub fn router(state: Arc<DataState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create))
        .with_state(state)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "",
    operation_id = "create_data",
    summary = "Create data",
    tag = constant::TAG_DATA,
    params(
        ("schema_name" = String, Path)
    ),
    request_body = Value,
    responses(
        (status = 200, body = Value)
    )
)]
async fn create(
    State(state): State<Arc<DataState>>,
    Path(schema_name): Path<String>,
    Json(data): Json<Value>,
) -> Result<Json<Value>, Error> {
    let req = CreateDataRequest { schema_name, data };
    let schema = state.data_service.create(&req).await?;
    Ok(Json(schema))
}
