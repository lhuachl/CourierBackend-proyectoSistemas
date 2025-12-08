use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::json;
use std::env;

use crate::domain::auth::{Claims, AuthenticatedUser};

/// Extrae el token del header Authorization
fn extract_token(request: &Request) -> Option<String> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                None
            }
        })
}

/// Valida el JWT usando el secret de Supabase
fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("SUPABASE_JWT_SECRET")
        .expect("SUPABASE_JWT_SECRET must be set");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

/// Middleware que requiere autenticación
pub async fn require_auth(mut request: Request, next: Next) -> Response {
    // Extraer token
    let token = match extract_token(&request) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "code": "TOKEN_MISSING",
                        "message": "Authorization header is required"
                    }
                })),
            ).into_response();
        }
    };

    // Validar token
    let claims = match validate_jwt(&token) {
        Ok(c) => c,
        Err(e) => {
            let (code, message) = match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    ("TOKEN_EXPIRED", "Token has expired")
                }
                _ => ("TOKEN_INVALID", "Invalid token"),
            };

            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "code": code,
                        "message": message
                    }
                })),
            ).into_response();
        }
    };

    // Verificar expiración
    if claims.is_expired() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": {
                    "code": "TOKEN_EXPIRED",
                    "message": "Token has expired"
                }
            })),
        ).into_response();
    }

    // Insertar usuario autenticado en las extensiones del request
    let user: AuthenticatedUser = claims.into();
    request.extensions_mut().insert(user);

    next.run(request).await
}

/// Middleware opcional - no falla si no hay token
pub async fn optional_auth(mut request: Request, next: Next) -> Response {
    if let Some(token) = extract_token(&request) {
        if let Ok(claims) = validate_jwt(&token) {
            if !claims.is_expired() {
                let user: AuthenticatedUser = claims.into();
                request.extensions_mut().insert(user);
            }
        }
    }

    next.run(request).await
}
