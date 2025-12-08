use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreatePedidoDTO, PedidoResponseDTO, PedidosListResponseDTO,
    UpdateEstadoPedidoDTO, AsignarTransportistaDTO,
};
use crate::application::services::PedidoService;
use crate::domain::auth::AuthenticatedUser;
use crate::shared::AppResult;

/// Listar pedidos del usuario autenticado
#[utoipa::path(
    get,
    path = "/api/pedidos",
    tag = "pedidos",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista de pedidos", body = PedidosListResponseDTO),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn list_pedidos(
    Extension(user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PedidoService>>,
) -> AppResult<Json<PedidosListResponseDTO>> {
    // TODO: Obtener id_perfil desde el user.id (requiere query a perfiles_cliente)
    // Por ahora usamos el user.id directamente
    let pedidos = service.list_by_user(user.id).await?;
    Ok(Json(pedidos))
}

/// Obtener pedido por ID
#[utoipa::path(
    get,
    path = "/api/pedidos/{id}",
    tag = "pedidos",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del pedido")
    ),
    responses(
        (status = 200, description = "Pedido encontrado", body = PedidoResponseDTO),
        (status = 404, description = "Pedido no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn get_pedido(
    Path(id): Path<Uuid>,
    Extension(_user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PedidoService>>,
) -> AppResult<Json<PedidoResponseDTO>> {
    let pedido = service.get_by_id(id).await?;
    Ok(Json(pedido))
}

/// Crear nuevo pedido
#[utoipa::path(
    post,
    path = "/api/pedidos",
    tag = "pedidos",
    security(("bearer_auth" = [])),
    request_body = CreatePedidoDTO,
    responses(
        (status = 201, description = "Pedido creado", body = PedidoResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn create_pedido(
    Extension(user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PedidoService>>,
    Json(dto): Json<CreatePedidoDTO>,
) -> AppResult<(StatusCode, Json<PedidoResponseDTO>)> {
    let pedido = service.create(user.id, dto).await?;
    Ok((StatusCode::CREATED, Json(pedido)))
}

/// Actualizar estado del pedido
#[utoipa::path(
    patch,
    path = "/api/pedidos/{id}/estado",
    tag = "pedidos",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del pedido")
    ),
    request_body = UpdateEstadoPedidoDTO,
    responses(
        (status = 200, description = "Estado actualizado", body = PedidoResponseDTO),
        (status = 400, description = "Transición de estado inválida"),
        (status = 404, description = "Pedido no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn update_estado_pedido(
    Path(id): Path<Uuid>,
    Extension(_user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PedidoService>>,
    Json(dto): Json<UpdateEstadoPedidoDTO>,
) -> AppResult<Json<PedidoResponseDTO>> {
    let pedido = service.update_estado(id, dto).await?;
    Ok(Json(pedido))
}

/// Asignar transportista al pedido
#[utoipa::path(
    patch,
    path = "/api/pedidos/{id}/transportista",
    tag = "pedidos",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del pedido")
    ),
    request_body = AsignarTransportistaDTO,
    responses(
        (status = 200, description = "Transportista asignado", body = PedidoResponseDTO),
        (status = 400, description = "No se puede asignar en este estado"),
        (status = 404, description = "Pedido no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn assign_transportista(
    Path(id): Path<Uuid>,
    Extension(_user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PedidoService>>,
    Json(dto): Json<AsignarTransportistaDTO>,
) -> AppResult<Json<PedidoResponseDTO>> {
    let pedido = service.assign_transportista(id, dto).await?;
    Ok(Json(pedido))
}

/// Cancelar pedido
#[utoipa::path(
    delete,
    path = "/api/pedidos/{id}",
    tag = "pedidos",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del pedido")
    ),
    responses(
        (status = 204, description = "Pedido cancelado"),
        (status = 400, description = "No se puede cancelar"),
        (status = 404, description = "Pedido no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn cancel_pedido(
    Path(id): Path<Uuid>,
    Extension(_user): Extension<AuthenticatedUser>,
    State(service): State<Arc<PedidoService>>,
) -> AppResult<StatusCode> {
    service.cancel(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
