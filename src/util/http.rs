use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

use super::error;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(error::Error))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl From<JsonRejection> for error::Error {
    fn from(err: JsonRejection) -> Self {
        error::invalid_argument_with_message(&err.to_string())
    }
}

impl IntoResponse for error::Error {
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let payload = json!({
            "message": self.message,
        });

        (code, axum::Json(payload)).into_response()
    }
}
