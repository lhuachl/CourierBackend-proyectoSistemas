use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateDireccionDTO, UpdateDireccionDTO, CreateAlmacenDTO,
    DireccionResponseDTO, DireccionesListResponseDTO,
};
use crate::application::services::DireccionService;
use crate::domain::auth::AuthenticatedUser;
use crate::domain::repositories::PerfilClienteRepository;
use crate::shared::error::{AppError, AppResult};

// ============================================================================
// HANDLERS - CLIENTE
// ============================================================================

/// Lista las direcciones del usuario autenticado
#[utoipa::path(
    get,
    path = "/api/direcciones",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de direcciones", body = DireccionesListResponseDTO),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn list_my_direcciones(
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<Json<DireccionesListResponseDTO>> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direcciones = service.list_my_direcciones(id_perfil).await?;
    Ok(Json(direcciones))
}

/// Obtiene una dirección por ID
#[utoipa::path(
    get,
    path = "/api/direcciones/{id}",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID de la dirección")
    ),
    responses(
        (status = 200, description = "Dirección encontrada", body = DireccionResponseDTO),
        (status = 404, description = "Dirección no encontrada"),
        (status = 403, description = "Sin acceso a esta dirección"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn get_direccion(
    Path(id): Path<Uuid>,
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<Json<DireccionResponseDTO>> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direccion = service.get_direccion(id, id_perfil).await?;
    Ok(Json(direccion))
}

/// Obtiene la dirección predeterminada del usuario
#[utoipa::path(
    get,
    path = "/api/direcciones/predeterminada",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Dirección predeterminada", body = DireccionResponseDTO),
        (status = 404, description = "No tiene dirección predeterminada"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn get_predeterminada(
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<Json<DireccionResponseDTO>> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direccion = service.get_predeterminada(id_perfil).await?;
    Ok(Json(direccion))
}

/// Crea una nueva dirección
#[utoipa::path(
    post,
    path = "/api/direcciones",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    request_body = CreateDireccionDTO,
    responses(
        (status = 201, description = "Dirección creada", body = DireccionResponseDTO),
        (status = 400, description = "Datos inválidos o límite alcanzado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn create_direccion(
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
    Json(dto): Json<CreateDireccionDTO>,
) -> AppResult<(StatusCode, Json<DireccionResponseDTO>)> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direccion = service.create_direccion(id_perfil, dto).await?;
    Ok((StatusCode::CREATED, Json(direccion)))
}

/// Actualiza una dirección
#[utoipa::path(
    put,
    path = "/api/direcciones/{id}",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID de la dirección")
    ),
    request_body = UpdateDireccionDTO,
    responses(
        (status = 200, description = "Dirección actualizada", body = DireccionResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 403, description = "Sin acceso"),
        (status = 404, description = "No encontrada"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn update_direccion(
    Path(id): Path<Uuid>,
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
    Json(dto): Json<UpdateDireccionDTO>,
) -> AppResult<Json<DireccionResponseDTO>> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direccion = service.update_direccion(id, id_perfil, dto).await?;
    Ok(Json(direccion))
}

/// Establece una dirección como predeterminada
#[utoipa::path(
    patch,
    path = "/api/direcciones/{id}/predeterminada",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID de la dirección")
    ),
    responses(
        (status = 200, description = "Dirección establecida como predeterminada", body = DireccionResponseDTO),
        (status = 400, description = "No se puede establecer una dirección inactiva"),
        (status = 403, description = "Sin acceso"),
        (status = 404, description = "No encontrada"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn set_predeterminada(
    Path(id): Path<Uuid>,
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<Json<DireccionResponseDTO>> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direccion = service.set_predeterminada(id, id_perfil).await?;
    Ok(Json(direccion))
}

/// Desactiva una dirección (soft delete)
#[utoipa::path(
    delete,
    path = "/api/direcciones/{id}",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID de la dirección")
    ),
    responses(
        (status = 204, description = "Dirección desactivada"),
        (status = 403, description = "Sin acceso"),
        (status = 404, description = "No encontrada"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn deactivate_direccion(
    Path(id): Path<Uuid>,
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<StatusCode> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    service.deactivate_direccion(id, id_perfil).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Reactiva una dirección
#[utoipa::path(
    patch,
    path = "/api/direcciones/{id}/activar",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID de la dirección")
    ),
    responses(
        (status = 200, description = "Dirección reactivada", body = DireccionResponseDTO),
        (status = 403, description = "Sin acceso"),
        (status = 404, description = "No encontrada"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn activate_direccion(
    Path(id): Path<Uuid>,
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<Json<DireccionResponseDTO>> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    let direccion = service.activate_direccion(id, id_perfil).await?;
    Ok(Json(direccion))
}

/// Elimina una dirección permanentemente (hard delete)
#[utoipa::path(
    delete,
    path = "/api/direcciones/{id}/permanente",
    tag = "direcciones",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID de la dirección")
    ),
    responses(
        (status = 204, description = "Dirección eliminada permanentemente"),
        (status = 403, description = "Sin acceso"),
        (status = 404, description = "No encontrada"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn delete_direccion_permanente(
    Path(id): Path<Uuid>,
    Extension(user): Extension<AuthenticatedUser>,
    State((service, perfil_repo)): State<(Arc<DireccionService>, Arc<dyn PerfilClienteRepository>)>,
) -> AppResult<StatusCode> {
    let id_perfil = get_perfil_id(&user, &perfil_repo).await?;
    service.delete_direccion(id, id_perfil).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// HANDLERS - ALMACENES (público para lectura)
// ============================================================================

/// Lista los almacenes disponibles
#[utoipa::path(
    get,
    path = "/api/almacenes",
    tag = "almacenes",
    responses(
        (status = 200, description = "Lista de almacenes", body = DireccionesListResponseDTO)
    )
)]
pub async fn list_almacenes(
    State(service): State<Arc<DireccionService>>,
) -> AppResult<Json<DireccionesListResponseDTO>> {
    let almacenes = service.list_almacenes().await?;
    Ok(Json(almacenes))
}

// ============================================================================
// HANDLERS - ADMIN (almacenes)
// ============================================================================

/// Lista todos los almacenes (incluye inactivos) - Admin
#[utoipa::path(
    get,
    path = "/api/admin/almacenes",
    tag = "almacenes-admin",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista completa de almacenes", body = DireccionesListResponseDTO),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn list_all_almacenes(
    State(service): State<Arc<DireccionService>>,
) -> AppResult<Json<DireccionesListResponseDTO>> {
    let almacenes = service.list_all_almacenes().await?;
    Ok(Json(almacenes))
}

/// Crea un nuevo almacén - Admin
#[utoipa::path(
    post,
    path = "/api/admin/almacenes",
    tag = "almacenes-admin",
    security(("bearer_auth" = [])),
    request_body = CreateAlmacenDTO,
    responses(
        (status = 201, description = "Almacén creado", body = DireccionResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn create_almacen(
    State(service): State<Arc<DireccionService>>,
    Json(dto): Json<CreateAlmacenDTO>,
) -> AppResult<(StatusCode, Json<DireccionResponseDTO>)> {
    let almacen = service.create_almacen(dto).await?;
    Ok((StatusCode::CREATED, Json(almacen)))
}

/// Desactiva un almacén - Admin
#[utoipa::path(
    delete,
    path = "/api/admin/almacenes/{id}",
    tag = "almacenes-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del almacén")
    ),
    responses(
        (status = 204, description = "Almacén desactivado"),
        (status = 400, description = "No es un almacén"),
        (status = 404, description = "No encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn deactivate_almacen(
    Path(id): Path<Uuid>,
    State(service): State<Arc<DireccionService>>,
) -> AppResult<StatusCode> {
    service.deactivate_almacen(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Reactiva un almacén - Admin
#[utoipa::path(
    patch,
    path = "/api/admin/almacenes/{id}/activar",
    tag = "almacenes-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del almacén")
    ),
    responses(
        (status = 200, description = "Almacén reactivado", body = DireccionResponseDTO),
        (status = 400, description = "No es un almacén"),
        (status = 404, description = "No encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn activate_almacen(
    Path(id): Path<Uuid>,
    State(service): State<Arc<DireccionService>>,
) -> AppResult<Json<DireccionResponseDTO>> {
    let almacen = service.activate_almacen(id).await?;
    Ok(Json(almacen))
}

/// Elimina un almacén permanentemente - Admin
#[utoipa::path(
    delete,
    path = "/api/admin/almacenes/{id}/permanente",
    tag = "almacenes-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del almacén")
    ),
    responses(
        (status = 204, description = "Almacén eliminado permanentemente"),
        (status = 400, description = "No es un almacén"),
        (status = 404, description = "No encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn delete_almacen_permanente(
    Path(id): Path<Uuid>,
    State(service): State<Arc<DireccionService>>,
) -> AppResult<StatusCode> {
    service.delete_almacen(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// HELPERS
// ============================================================================

/// Obtiene el id_perfil del usuario autenticado
async fn get_perfil_id(
    user: &AuthenticatedUser,
    perfil_repo: &Arc<dyn PerfilClienteRepository>,
) -> AppResult<Uuid> {
    let perfil = perfil_repo
        .find_by_usuario(user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Debe crear un perfil primero".into()))?;
    
    Ok(perfil.id_perfil)
}
