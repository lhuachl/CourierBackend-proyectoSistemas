use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateUserDTO, UpdateUserDTO, UpdateUserRoleDTO, UpdateUserStatusDTO,
    UserResponseDTO, UsersListResponseDTO,
};
use crate::application::services::UserService;
use crate::domain::auth::AuthenticatedUser;
use crate::shared::error::AppResult;

// ============================================================================
// HANDLERS - ADMIN
// ============================================================================

/// Lista todos los usuarios (admin)
#[utoipa::path(
    get,
    path = "/api/admin/users",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de usuarios", body = UsersListResponseDTO),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn list_users(
    Extension(_user): Extension<AuthenticatedUser>,
    State(service): State<Arc<UserService>>,
) -> AppResult<Json<UsersListResponseDTO>> {
    let users = service.list_users().await?;
    Ok(Json(users))
}

/// Obtiene un usuario por ID (admin)
#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del usuario")
    ),
    responses(
        (status = 200, description = "Usuario encontrado", body = UserResponseDTO),
        (status = 404, description = "Usuario no encontrado"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn get_user(
    Extension(_user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    State(service): State<Arc<UserService>>,
) -> AppResult<Json<UserResponseDTO>> {
    let user = service.get_user(id).await?;
    Ok(Json(user))
}

/// Crea un nuevo usuario (admin - para sincronización con Supabase)
#[utoipa::path(
    post,
    path = "/api/admin/users",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    request_body = CreateUserDTO,
    responses(
        (status = 201, description = "Usuario creado", body = UserResponseDTO),
        (status = 400, description = "Datos inválidos o usuario ya existe"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn create_user(
    Extension(_user): Extension<AuthenticatedUser>,
    State(service): State<Arc<UserService>>,
    Json(dto): Json<CreateUserDTO>,
) -> AppResult<(StatusCode, Json<UserResponseDTO>)> {
    let user = service.create_user(dto).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// Actualiza un usuario (admin)
#[utoipa::path(
    put,
    path = "/api/admin/users/{id}",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del usuario")
    ),
    request_body = UpdateUserDTO,
    responses(
        (status = 200, description = "Usuario actualizado", body = UserResponseDTO),
        (status = 404, description = "Usuario no encontrado"),
        (status = 400, description = "Datos inválidos"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn update_user(
    Extension(_user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    State(service): State<Arc<UserService>>,
    Json(dto): Json<UpdateUserDTO>,
) -> AppResult<Json<UserResponseDTO>> {
    let user = service.update_user(id, dto).await?;
    Ok(Json(user))
}

/// Actualiza el rol de un usuario (solo admin)
#[utoipa::path(
    patch,
    path = "/api/admin/users/{id}/role",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del usuario")
    ),
    request_body = UpdateUserRoleDTO,
    responses(
        (status = 200, description = "Rol actualizado", body = UserResponseDTO),
        (status = 404, description = "Usuario no encontrado"),
        (status = 400, description = "Rol inválido"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn update_user_role(
    Extension(_user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    State(service): State<Arc<UserService>>,
    Json(dto): Json<UpdateUserRoleDTO>,
) -> AppResult<Json<UserResponseDTO>> {
    let user = service.update_user_role(id, dto).await?;
    Ok(Json(user))
}

/// Actualiza el estado de un usuario (solo admin)
#[utoipa::path(
    patch,
    path = "/api/admin/users/{id}/status",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del usuario")
    ),
    request_body = UpdateUserStatusDTO,
    responses(
        (status = 200, description = "Estado actualizado", body = UserResponseDTO),
        (status = 404, description = "Usuario no encontrado"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn update_user_status(
    Extension(_user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    State(service): State<Arc<UserService>>,
    Json(dto): Json<UpdateUserStatusDTO>,
) -> AppResult<Json<UserResponseDTO>> {
    let user = service.update_user_status(id, dto).await?;
    Ok(Json(user))
}

/// Elimina un usuario (soft delete - solo admin)
#[utoipa::path(
    delete,
    path = "/api/admin/users/{id}",
    tag = "usuarios-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del usuario")
    ),
    responses(
        (status = 204, description = "Usuario eliminado"),
        (status = 404, description = "Usuario no encontrado"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "No es administrador")
    )
)]
pub async fn delete_user(
    Extension(_user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    State(service): State<Arc<UserService>>,
) -> AppResult<StatusCode> {
    service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
