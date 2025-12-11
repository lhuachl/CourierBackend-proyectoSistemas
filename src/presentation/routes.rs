use axum::{
    middleware,
    routing::{get, patch, delete, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::openapi::security::{SecurityScheme, Http, HttpAuthScheme};

use crate::application::dto::{
    CreateUserDTO, UpdateUserDTO, UserResponseDTO, UpdateUserRoleDTO, UpdateUserStatusDTO, UsersListResponseDTO,
    CreateProductoDTO, UpdateProductoDTO, UpdateStockDTO, UpdateEstadoProductoDTO,
    ProductoResponseDTO, ProductosListResponseDTO,
    CreatePedidoDTO, PedidoResponseDTO, PedidosListResponseDTO,
    UpdateEstadoPedidoDTO, AsignarTransportistaDTO,
    CreatePerfilClienteDTO, UpdatePerfilClienteDTO, 
    PerfilClienteResponseDTO, PerfilesClienteListResponseDTO,
    CreateDireccionDTO, UpdateDireccionDTO, CreateAlmacenDTO,
    DireccionResponseDTO, DireccionesListResponseDTO,
};
use crate::application::services::{UserService, PedidoService, PerfilClienteService, ProductoService, DireccionService};
use crate::domain::repositories::{UserRepository, PedidoRepository, PerfilClienteRepository, ProductoRepository, DireccionRepository};
use crate::infrastructure::repositories::{UserRepositoryImpl, PedidoRepositoryImpl, PerfilClienteRepositoryImpl, ProductoRepositoryImpl, DireccionRepositoryImpl};
use crate::presentation::handlers::{
    get_current_user, CurrentUserResponse, __path_get_current_user,
    list_users, get_user, create_user, update_user, update_user_role, update_user_status, delete_user,
    __path_list_users, __path_get_user, __path_create_user, __path_update_user, __path_update_user_role, __path_update_user_status, __path_delete_user,
    list_pedidos, get_pedido, create_pedido, 
    update_estado_pedido, assign_transportista, cancel_pedido,
    __path_list_pedidos, __path_get_pedido, __path_create_pedido,
    __path_update_estado_pedido, __path_assign_transportista, __path_cancel_pedido,
    get_my_perfil, create_perfil, update_my_perfil, delete_my_perfil,
    list_perfiles, get_perfil_by_id, update_perfil_by_id, delete_perfil_by_id,
    __path_get_my_perfil, __path_create_perfil, __path_update_my_perfil, __path_delete_my_perfil,
    __path_list_perfiles, __path_get_perfil_by_id, __path_update_perfil_by_id, __path_delete_perfil_by_id,
    list_productos, get_producto, search_productos, get_by_categoria, get_by_sku,
    list_all_productos, create_producto, update_producto, update_stock,
    update_estado_producto, delete_producto,
    __path_list_productos, __path_get_producto, __path_search_productos,
    __path_get_by_categoria, __path_get_by_sku,
    __path_list_all_productos, __path_create_producto, __path_update_producto,
    __path_update_stock, __path_update_estado_producto, __path_delete_producto,
    list_my_direcciones, get_direccion, get_predeterminada, create_direccion,
    update_direccion, set_predeterminada, deactivate_direccion, activate_direccion,
    delete_direccion_permanente, list_almacenes,
    list_all_almacenes, create_almacen, deactivate_almacen, activate_almacen, delete_almacen_permanente,
    __path_list_my_direcciones, __path_get_direccion, __path_get_predeterminada, __path_create_direccion,
    __path_update_direccion, __path_set_predeterminada, __path_deactivate_direccion, __path_activate_direccion,
    __path_delete_direccion_permanente, __path_list_almacenes,
    __path_list_all_almacenes, __path_create_almacen, __path_deactivate_almacen, __path_activate_almacen,
    __path_delete_almacen_permanente,
};
use crate::presentation::middleware::require_auth;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Integrador API",
        version = "1.0.0",
        description = "API Backend para sistema de logística y entregas",
        contact(name = "Soporte", email = "soporte@integrador.com")
    ),
    servers(
        (url = "http://localhost:3000", description = "Desarrollo local"),
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Autenticación y autorización"),
        (name = "usuarios-admin", description = "Gestión de usuarios (admin)"),
        (name = "perfiles", description = "Gestión de perfil de cliente"),
        (name = "perfiles-admin", description = "Gestión de perfiles (admin)"),
        (name = "productos", description = "Catálogo de productos (público)"),
        (name = "productos-admin", description = "Gestión de productos (admin)"),
        (name = "pedidos", description = "Gestión de pedidos"),
        (name = "direcciones", description = "Gestión de direcciones del cliente"),
        (name = "almacenes", description = "Almacenes (lectura pública)"),
        (name = "almacenes-admin", description = "Gestión de almacenes (admin)"),
    ),
    components(
        schemas(
            CreateUserDTO, UpdateUserDTO, UserResponseDTO, UpdateUserRoleDTO, UpdateUserStatusDTO, UsersListResponseDTO,
            CreateProductoDTO, UpdateProductoDTO, UpdateStockDTO, UpdateEstadoProductoDTO,
            ProductoResponseDTO, ProductosListResponseDTO,
            CreatePedidoDTO, PedidoResponseDTO, PedidosListResponseDTO,
            UpdateEstadoPedidoDTO, AsignarTransportistaDTO,
            CreatePerfilClienteDTO, UpdatePerfilClienteDTO,
            PerfilClienteResponseDTO, PerfilesClienteListResponseDTO,
            CreateDireccionDTO, UpdateDireccionDTO, CreateAlmacenDTO,
            DireccionResponseDTO, DireccionesListResponseDTO,
            CurrentUserResponse,
        )
    ),
    paths(
        health_check,
        get_current_user,
        // Usuarios admin
        list_users,
        get_user,
        create_user,
        update_user,
        update_user_role,
        update_user_status,
        delete_user,
        // Pedidos
        list_pedidos,
        get_pedido,
        create_pedido,
        update_estado_pedido,
        assign_transportista,
        cancel_pedido,
        // Perfiles
        get_my_perfil,
        create_perfil,
        update_my_perfil,
        delete_my_perfil,
        list_perfiles,
        get_perfil_by_id,
        update_perfil_by_id,
        delete_perfil_by_id,
        // Productos públicos
        list_productos,
        get_producto,
        search_productos,
        get_by_categoria,
        get_by_sku,
        // Productos admin
        list_all_productos,
        create_producto,
        update_producto,
        update_stock,
        update_estado_producto,
        delete_producto,
        // Direcciones cliente
        list_my_direcciones,
        get_direccion,
        get_predeterminada,
        create_direccion,
        update_direccion,
        set_predeterminada,
        deactivate_direccion,
        activate_direccion,
        delete_direccion_permanente,
        // Almacenes públicos
        list_almacenes,
        // Almacenes admin
        list_all_almacenes,
        create_almacen,
        deactivate_almacen,
        activate_almacen,
        delete_almacen_permanente,
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}

pub fn create_routes(pool: PgPool) -> Router {
    // Crear repositorio y service de usuarios (Dependency Injection)
    let user_repo: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(pool.clone()));
    let user_service = Arc::new(UserService::new(user_repo));

    // Crear repositorio y service de pedidos (Dependency Injection)
    let pedido_repo: Arc<dyn PedidoRepository> = Arc::new(PedidoRepositoryImpl::new(pool.clone()));
    let pedido_service = Arc::new(PedidoService::new(pedido_repo));

    // Crear repositorio y service de perfiles de cliente (Dependency Injection)
    let perfil_repo: Arc<dyn PerfilClienteRepository> = Arc::new(PerfilClienteRepositoryImpl::new(pool.clone()));
    let perfil_service = Arc::new(PerfilClienteService::new(perfil_repo.clone()));

    // Crear repositorio y service de productos (Dependency Injection)
    let producto_repo: Arc<dyn ProductoRepository> = Arc::new(ProductoRepositoryImpl::new(pool.clone()));
    let producto_service = Arc::new(ProductoService::new(producto_repo));

    // Crear repositorio y service de direcciones (Dependency Injection)
    let direccion_repo: Arc<dyn DireccionRepository> = Arc::new(DireccionRepositoryImpl::new(pool.clone()));
    let direccion_service = Arc::new(DireccionService::new(direccion_repo));

    // Rutas admin de usuarios (protegidas)
    let admin_users_routes = Router::new()
        .route("/api/admin/users", get(list_users).post(create_user))
        .route("/api/admin/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/api/admin/users/{id}/role", patch(update_user_role))
        .route("/api/admin/users/{id}/status", patch(update_user_status))
        .with_state(user_service)
        .route_layer(middleware::from_fn(require_auth));

    // Rutas de pedidos (protegidas)
    let pedidos_routes = Router::new()
        .route("/api/pedidos", get(list_pedidos).post(create_pedido))
        .route("/api/pedidos/{id}", get(get_pedido).delete(cancel_pedido))
        .route("/api/pedidos/{id}/estado", patch(update_estado_pedido))
        .route("/api/pedidos/{id}/transportista", patch(assign_transportista))
        .with_state(pedido_service)
        .route_layer(middleware::from_fn(require_auth));

    // Rutas de perfil de cliente (protegidas)
    let perfil_routes = Router::new()
        .route("/api/perfil", get(get_my_perfil).post(create_perfil).put(update_my_perfil).delete(delete_my_perfil))
        .with_state(perfil_service.clone())
        .route_layer(middleware::from_fn(require_auth));

    // Rutas admin de perfiles (protegidas - TODO: agregar middleware de rol admin)
    let admin_perfil_routes = Router::new()
        .route("/api/admin/perfiles", get(list_perfiles))
        .route("/api/admin/perfiles/{id}", get(get_perfil_by_id).put(update_perfil_by_id).delete(delete_perfil_by_id))
        .with_state(perfil_service)
        .route_layer(middleware::from_fn(require_auth));

    // Rutas públicas de productos (catálogo)
    let productos_public_routes = Router::new()
        .route("/api/productos", get(list_productos))
        .route("/api/productos/buscar", get(search_productos))
        .route("/api/productos/categoria/{categoria}", get(get_by_categoria))
        .route("/api/productos/sku/{sku}", get(get_by_sku))
        .route("/api/productos/{id}", get(get_producto))
        .with_state(producto_service.clone());

    // Rutas admin de productos (protegidas)
    let admin_productos_routes = Router::new()
        .route("/api/admin/productos", get(list_all_productos).post(create_producto))
        .route("/api/admin/productos/{id}", put(update_producto).delete(delete_producto))
        .route("/api/admin/productos/{id}/stock", patch(update_stock))
        .route("/api/admin/productos/{id}/estado", patch(update_estado_producto))
        .with_state(producto_service)
        .route_layer(middleware::from_fn(require_auth));

    // Rutas de direcciones del cliente (protegidas)
    let direcciones_routes = Router::new()
        .route("/api/direcciones", get(list_my_direcciones).post(create_direccion))
        .route("/api/direcciones/predeterminada", get(get_predeterminada))
        .route("/api/direcciones/{id}", get(get_direccion).put(update_direccion).delete(deactivate_direccion))
        .route("/api/direcciones/{id}/predeterminada", patch(set_predeterminada))
        .route("/api/direcciones/{id}/activar", patch(activate_direccion))
        .route("/api/direcciones/{id}/permanente", delete(delete_direccion_permanente))
        .with_state((direccion_service.clone(), perfil_repo.clone()))
        .route_layer(middleware::from_fn(require_auth));

    // Rutas públicas de almacenes
    let almacenes_public_routes = Router::new()
        .route("/api/almacenes", get(list_almacenes))
        .with_state(direccion_service.clone());

    // Rutas admin de almacenes (protegidas)
    let admin_almacenes_routes = Router::new()
        .route("/api/admin/almacenes", get(list_all_almacenes).post(create_almacen))
        .route("/api/admin/almacenes/{id}", delete(deactivate_almacen))
        .route("/api/admin/almacenes/{id}/activar", patch(activate_almacen))
        .route("/api/admin/almacenes/{id}/permanente", delete(delete_almacen_permanente))
        .with_state(direccion_service)
        .route_layer(middleware::from_fn(require_auth));

    // Rutas de auth (protegidas)
    let auth_routes = Router::new()
        .route("/auth/me", get(get_current_user))
        .route_layer(middleware::from_fn(require_auth));

    // Rutas públicas
    let public_routes = Router::new()
        .route("/health", get(health_check));

    // Combinar todas las rutas
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(public_routes)
        .merge(auth_routes)
        .merge(admin_users_routes)
        .merge(pedidos_routes)
        .merge(perfil_routes)
        .merge(admin_perfil_routes)
        .merge(productos_public_routes)
        .merge(admin_productos_routes)
        .merge(direcciones_routes)
        .merge(almacenes_public_routes)
        .merge(admin_almacenes_routes)
        .with_state(pool)
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Servidor funcionando", body = String)
    )
)]
pub async fn health_check() -> &'static str {
    "OK"
}
