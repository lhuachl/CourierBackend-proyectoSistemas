use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreatePerfilClienteDTO, UpdatePerfilClienteDTO, 
    PerfilClienteResponseDTO, PerfilesClienteListResponseDTO,
};
use crate::application::services::PerfilClienteService;
use crate::domain::auth::AuthenticatedUser;
use crate::shared::error::AppResult;

// ============================================================================
// HANDLERS
// ============================================================================

/// Obtiene el perfil del usuario autenticado
#[utoipa::path(
    get,
    path = "/api/perfil",
    tag = "perfiles",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Perfil encontrado", body = PerfilClienteResponseDTO),
        (status = 404, description = "Perfil no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn get_my_perfil(
    Extension(user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PerfilClienteService>>,
) -> AppResult<Json<PerfilClienteResponseDTO>> {
    let perfil = service.get_my_perfil(user.id).await?;
    Ok(Json(perfil))
}

/// Crea un perfil para el usuario autenticado
#[utoipa::path(
    post,
    path = "/api/perfil",
    tag = "perfiles",
    security(("bearer_auth" = [])),
    request_body = CreatePerfilClienteDTO,
    responses(
        (status = 201, description = "Perfil creado", body = PerfilClienteResponseDTO),
        (status = 400, description = "Datos inválidos o perfil ya existe"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn create_perfil(
    Extension(user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PerfilClienteService>>,
    Json(dto): Json<CreatePerfilClienteDTO>,
) -> AppResult<(StatusCode, Json<PerfilClienteResponseDTO>)> {
    let perfil = service.create_perfil(user.id, dto).await?;
    Ok((StatusCode::CREATED, Json(perfil)))
}

/// Actualiza el perfil del usuario autenticado
#[utoipa::path(
    put,
    path = "/api/perfil",
    tag = "perfiles",
    security(("bearer_auth" = [])),
    request_body = UpdatePerfilClienteDTO,
    responses(
        (status = 200, description = "Perfil actualizado", body = PerfilClienteResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 404, description = "Perfil no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn update_my_perfil(
    Extension(user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PerfilClienteService>>,
    Json(dto): Json<UpdatePerfilClienteDTO>,
) -> AppResult<Json<PerfilClienteResponseDTO>> {
    let perfil = service.update_my_perfil(user.id, dto).await?;
    Ok(Json(perfil))
}

/// Elimina el perfil del usuario autenticado
#[utoipa::path(
    delete,
    path = "/api/perfil",
    tag = "perfiles",
    security(("bearer_auth" = [])),
    responses(
        (status = 204, description = "Perfil eliminado"),
        (status = 404, description = "Perfil no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn delete_my_perfil(
    Extension(user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PerfilClienteService>>,
) -> AppResult<StatusCode> {
    service.delete_my_perfil(user.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// HANDLERS ADMIN
// ============================================================================

/// Lista todos los perfiles (solo admin)
#[utoipa::path(
    get,
    path = "/api/admin/perfiles",
    tag = "perfiles-admin",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de perfiles", body = PerfilesClienteListResponseDTO),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn list_perfiles(
    State(service): State<Arc<PerfilClienteService>>,
) -> AppResult<Json<PerfilesClienteListResponseDTO>> {
    let perfiles = service.list_perfiles().await?;
    Ok(Json(perfiles))
}

/// Obtiene un perfil por ID (solo admin)
#[utoipa::path(
    get,
    path = "/api/admin/perfiles/{id}",
    tag = "perfiles-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del perfil")
    ),
    responses(
        (status = 200, description = "Perfil encontrado", body = PerfilClienteResponseDTO),
        (status = 404, description = "Perfil no encontrado"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn get_perfil_by_id(
    Path(id): Path<Uuid>,
    State(service): State<Arc<PerfilClienteService>>,
) -> AppResult<Json<PerfilClienteResponseDTO>> {
    let perfil = service.get_perfil_by_id(id).await?;
    Ok(Json(perfil))
}

/// Actualiza un perfil por ID (solo admin)
#[utoipa::path(
    put,
    path = "/api/admin/perfiles/{id}",
    tag = "perfiles-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del perfil")
    ),
    request_body = UpdatePerfilClienteDTO,
    responses(
        (status = 200, description = "Perfil actualizado", body = PerfilClienteResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 404, description = "Perfil no encontrado"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn update_perfil_by_id(
    Path(id): Path<Uuid>,
    State(service): State<Arc<PerfilClienteService>>,
    Json(dto): Json<UpdatePerfilClienteDTO>,
) -> AppResult<Json<PerfilClienteResponseDTO>> {
    let perfil = service.update_perfil_by_id(id, dto).await?;
    Ok(Json(perfil))
}

/// Elimina un perfil por ID (solo admin)
#[utoipa::path(
    delete,
    path = "/api/admin/perfiles/{id}",
    tag = "perfiles-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del perfil")
    ),
    responses(
        (status = 204, description = "Perfil eliminado"),
        (status = 404, description = "Perfil no encontrado"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn delete_perfil_by_id(
    Path(id): Path<Uuid>,
    State(service): State<Arc<PerfilClienteService>>,
) -> AppResult<StatusCode> {
    service.delete_perfil_by_id(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
