use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::auth::AuthenticatedUser;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrentUserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub role: Option<String>,
}

/// Obtener usuario actual autenticado
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Usuario autenticado", body = CurrentUserResponse),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn get_current_user(
    Extension(user): Extension<AuthenticatedUser>,
) -> (StatusCode, Json<CurrentUserResponse>) {
    (
        StatusCode::OK,
        Json(CurrentUserResponse {
            id: user.id,
            email: user.email,
            role: user.role,
        }),
    )
}
