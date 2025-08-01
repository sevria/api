use std::sync::Arc;

use axum::extract::State;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    constant,
    domain::auth::model::{LoginRequest, LoginResponse, RefreshTokenRequest},
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
        .routes(routes!(refresh))
        .with_state(state)
}

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

#[utoipa::path(
    post,
    path = "/refresh",
    summary = "Refresh token",
    tag = constant::TAG_AUTH,
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, body = LoginResponse)
    )
)]
async fn refresh(
    State(state): State<Arc<AuthState>>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<Json<LoginResponse>, Error> {
    match state.auth_service.refresh(&req).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(err),
    }
}
