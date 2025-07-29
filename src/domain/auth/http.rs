use std::sync::Arc;

use axum::extract::State;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    domain::auth::model::{LoginRequest, LoginResponse},
    util::{error::Error, http::Json},
};

use super::service::AuthService;

pub struct AuthState {
    pub auth_service: Arc<AuthService>,
}

impl AuthState {
    pub fn new(auth_service: Arc<AuthService>) -> AuthState {
        AuthState { auth_service }
    }
}

pub fn router(state: Arc<AuthState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(login))
        .with_state(state)
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/login",
    summary = "Login",
    tag = constant::TAG_AUTH,
    request_body = LoginRequest,
    responses(
        (status = 200, body = LoginResponse)
    )
)]
async fn login(
    State(state): State<Arc<AuthState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Error> {
    let res = state.auth_service.login(&req).await?;
    Ok(Json(res))
}
