pub mod user_handler;
pub mod producto_handler;
pub mod pedido_handler;
pub mod auth_handler;
pub mod perfil_cliente_handler;
pub mod direccion_handler;

pub use auth_handler::{get_current_user, CurrentUserResponse, __path_get_current_user};
pub use user_handler::{
    list_users, get_user, create_user, update_user, update_user_role, update_user_status, delete_user,
    __path_list_users, __path_get_user, __path_create_user, __path_update_user, __path_update_user_role, __path_update_user_status, __path_delete_user,
};
pub use pedido_handler::{
    list_pedidos, get_pedido, create_pedido, 
    update_estado_pedido, assign_transportista, cancel_pedido,
    __path_list_pedidos, __path_get_pedido, __path_create_pedido,
    __path_update_estado_pedido, __path_assign_transportista, __path_cancel_pedido,
};
pub use perfil_cliente_handler::{
    get_my_perfil, create_perfil, update_my_perfil, delete_my_perfil,
    list_perfiles, get_perfil_by_id, update_perfil_by_id, delete_perfil_by_id,
    __path_get_my_perfil, __path_create_perfil, __path_update_my_perfil, __path_delete_my_perfil,
    __path_list_perfiles, __path_get_perfil_by_id, __path_update_perfil_by_id, __path_delete_perfil_by_id,
};
pub use producto_handler::{
    list_productos, get_producto, search_productos, get_by_categoria, get_by_sku,
    list_all_productos, create_producto, update_producto, update_stock, 
    update_estado_producto, delete_producto,
    __path_list_productos, __path_get_producto, __path_search_productos, 
    __path_get_by_categoria, __path_get_by_sku,
    __path_list_all_productos, __path_create_producto, __path_update_producto,
    __path_update_stock, __path_update_estado_producto, __path_delete_producto,
};
pub use direccion_handler::{
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
