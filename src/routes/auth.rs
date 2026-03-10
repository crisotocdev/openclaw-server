use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::auth;      // ✅ módulo auth real
use crate::auth::Rol; // ✅ enum Rol

#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub ok: bool,
    pub response: String,      // "TOKEN_OK" / "TOKEN_INVALID"
    pub role: Option<String>,  // "admin" / "user"
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub role: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify))
}

async fn login(Json(payload): Json<TokenRequest>) -> impl IntoResponse {
    let token = payload.token.trim();

    match auth::rol(token) {
        Some(Rol::Admin) => (
            StatusCode::OK,
            Json(LoginResponse { role: "admin".into() }),
        )
            .into_response(),
        Some(Rol::User) => (
            StatusCode::OK,
            Json(LoginResponse { role: "user".into() }),
        )
            .into_response(),
        None => (StatusCode::UNAUTHORIZED, "token invalido").into_response(),
    }
}

async fn verify(Json(payload): Json<TokenRequest>) -> impl IntoResponse {
    let token = payload.token.trim();

    println!("VERIFY: entro a verify_token");
    println!("VERIFY: token len={}", token.len());
    println!("VERIFY: llamando auth::rol()");

    match auth::rol(token) {
        Some(Rol::Admin) => {
            println!("VERIFY: rol=admin");
            (
                StatusCode::OK,
                Json(VerifyResponse {
                    ok: true,
                    response: "TOKEN_OK".into(),
                    role: Some("admin".into()),
                }),
            )
                .into_response()
        }
        Some(Rol::User) => {
            println!("VERIFY: rol=user");
            (
                StatusCode::OK,
                Json(VerifyResponse {
                    ok: true,
                    response: "TOKEN_OK".into(),
                    role: Some("user".into()),
                }),
            )
                .into_response()
        }
        None => {
            println!("VERIFY: token invalido");
            (
                StatusCode::UNAUTHORIZED,
                Json(VerifyResponse {
                    ok: false,
                    response: "TOKEN_INVALID".into(),
                    role: None,
                }),
            )
                .into_response()
        }
    }
}