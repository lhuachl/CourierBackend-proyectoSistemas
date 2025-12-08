use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateProductoDTO, UpdateProductoDTO, UpdateStockDTO, UpdateEstadoProductoDTO,
    ProductoResponseDTO, ProductosListResponseDTO,
};
use crate::application::services::ProductoService;
use crate::shared::error::AppResult;

// ============================================================================
// QUERY PARAMS
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CategoriaQuery {
    pub categoria: String,
}

// ============================================================================
// HANDLERS PÚBLICOS
// ============================================================================

/// Lista todos los productos activos
#[utoipa::path(
    get,
    path = "/api/productos",
    tag = "productos",
    responses(
        (status = 200, description = "Lista de productos", body = ProductosListResponseDTO)
    )
)]
pub async fn list_productos(
    State(service): State<Arc<ProductoService>>,
) -> AppResult<Json<ProductosListResponseDTO>> {
    let productos = service.list_productos().await?;
    Ok(Json(productos))
}

/// Obtiene un producto por ID
#[utoipa::path(
    get,
    path = "/api/productos/{id}",
    tag = "productos",
    params(
        ("id" = Uuid, Path, description = "ID del producto")
    ),
    responses(
        (status = 200, description = "Producto encontrado", body = ProductoResponseDTO),
        (status = 404, description = "Producto no encontrado")
    )
)]
pub async fn get_producto(
    Path(id): Path<Uuid>,
    State(service): State<Arc<ProductoService>>,
) -> AppResult<Json<ProductoResponseDTO>> {
    let producto = service.get_producto(id).await?;
    Ok(Json(producto))
}

/// Busca productos por término
#[utoipa::path(
    get,
    path = "/api/productos/buscar",
    tag = "productos",
    params(
        ("q" = String, Query, description = "Término de búsqueda")
    ),
    responses(
        (status = 200, description = "Resultados de búsqueda", body = ProductosListResponseDTO),
        (status = 400, description = "Término de búsqueda vacío")
    )
)]
pub async fn search_productos(
    Query(query): Query<SearchQuery>,
    State(service): State<Arc<ProductoService>>,
) -> AppResult<Json<ProductosListResponseDTO>> {
    let q = query.q.unwrap_or_default();
    let productos = service.search_productos(&q).await?;
    Ok(Json(productos))
}

/// Obtiene productos por categoría
#[utoipa::path(
    get,
    path = "/api/productos/categoria/{categoria}",
    tag = "productos",
    params(
        ("categoria" = String, Path, description = "Nombre de la categoría")
    ),
    responses(
        (status = 200, description = "Productos de la categoría", body = ProductosListResponseDTO)
    )
)]
pub async fn get_by_categoria(
    Path(categoria): Path<String>,
    State(service): State<Arc<ProductoService>>,
) -> AppResult<Json<ProductosListResponseDTO>> {
    let productos = service.get_by_categoria(&categoria).await?;
    Ok(Json(productos))
}

/// Obtiene un producto por SKU
#[utoipa::path(
    get,
    path = "/api/productos/sku/{sku}",
    tag = "productos",
    params(
        ("sku" = String, Path, description = "Código SKU del producto")
    ),
    responses(
        (status = 200, description = "Producto encontrado", body = ProductoResponseDTO),
        (status = 404, description = "Producto no encontrado")
    )
)]
pub async fn get_by_sku(
    Path(sku): Path<String>,
    State(service): State<Arc<ProductoService>>,
) -> AppResult<Json<ProductoResponseDTO>> {
    let producto = service.get_producto_by_sku(&sku).await?;
    Ok(Json(producto))
}

// ============================================================================
// HANDLERS ADMIN (requieren autenticación)
// ============================================================================

/// Lista todos los productos (incluye inactivos) - Admin
#[utoipa::path(
    get,
    path = "/api/admin/productos",
    tag = "productos-admin",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Lista completa de productos", body = ProductosListResponseDTO),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn list_all_productos(
    State(service): State<Arc<ProductoService>>,
) -> AppResult<Json<ProductosListResponseDTO>> {
    let productos = service.list_all_productos().await?;
    Ok(Json(productos))
}

/// Crea un nuevo producto - Admin
#[utoipa::path(
    post,
    path = "/api/admin/productos",
    tag = "productos-admin",
    security(("bearer_auth" = [])),
    request_body = CreateProductoDTO,
    responses(
        (status = 201, description = "Producto creado", body = ProductoResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 401, description = "No autenticado"),
        (status = 403, description = "Sin permisos")
    )
)]
pub async fn create_producto(
    State(service): State<Arc<ProductoService>>,
    Json(dto): Json<CreateProductoDTO>,
) -> AppResult<(StatusCode, Json<ProductoResponseDTO>)> {
    let producto = service.create_producto(dto).await?;
    Ok((StatusCode::CREATED, Json(producto)))
}

/// Actualiza un producto - Admin
#[utoipa::path(
    put,
    path = "/api/admin/productos/{id}",
    tag = "productos-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del producto")
    ),
    request_body = UpdateProductoDTO,
    responses(
        (status = 200, description = "Producto actualizado", body = ProductoResponseDTO),
        (status = 400, description = "Datos inválidos"),
        (status = 404, description = "Producto no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn update_producto(
    Path(id): Path<Uuid>,
    State(service): State<Arc<ProductoService>>,
    Json(dto): Json<UpdateProductoDTO>,
) -> AppResult<Json<ProductoResponseDTO>> {
    let producto = service.update_producto(id, dto).await?;
    Ok(Json(producto))
}

/// Actualiza el stock de un producto - Admin
#[utoipa::path(
    patch,
    path = "/api/admin/productos/{id}/stock",
    tag = "productos-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del producto")
    ),
    request_body = UpdateStockDTO,
    responses(
        (status = 200, description = "Stock actualizado", body = ProductoResponseDTO),
        (status = 400, description = "Stock insuficiente o datos inválidos"),
        (status = 404, description = "Producto no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn update_stock(
    Path(id): Path<Uuid>,
    State(service): State<Arc<ProductoService>>,
    Json(dto): Json<UpdateStockDTO>,
) -> AppResult<Json<ProductoResponseDTO>> {
    let producto = service.update_stock(id, dto).await?;
    Ok(Json(producto))
}

/// Cambia el estado de un producto (activar/desactivar) - Admin
#[utoipa::path(
    patch,
    path = "/api/admin/productos/{id}/estado",
    tag = "productos-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del producto")
    ),
    request_body = UpdateEstadoProductoDTO,
    responses(
        (status = 200, description = "Estado actualizado", body = ProductoResponseDTO),
        (status = 404, description = "Producto no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn update_estado_producto(
    Path(id): Path<Uuid>,
    State(service): State<Arc<ProductoService>>,
    Json(dto): Json<UpdateEstadoProductoDTO>,
) -> AppResult<Json<ProductoResponseDTO>> {
    let producto = service.update_estado(id, dto).await?;
    Ok(Json(producto))
}

/// Elimina un producto - Admin
#[utoipa::path(
    delete,
    path = "/api/admin/productos/{id}",
    tag = "productos-admin",
    security(("bearer_auth" = [])),
    params(
        ("id" = Uuid, Path, description = "ID del producto")
    ),
    responses(
        (status = 204, description = "Producto eliminado"),
        (status = 404, description = "Producto no encontrado"),
        (status = 401, description = "No autenticado")
    )
)]
pub async fn delete_producto(
    Path(id): Path<Uuid>,
    State(service): State<Arc<ProductoService>>,
) -> AppResult<StatusCode> {
    service.delete_producto(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
