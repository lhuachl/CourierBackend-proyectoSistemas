use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Entidad Producto - Representa un producto del catálogo
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Producto {
    /// ID único del producto
    pub id_producto: Uuid,
    /// Nombre del producto
    pub nombre_producto: String,
    /// Descripción detallada
    pub descripcion: Option<String>,
    /// Precio unitario (CHECK: precio > 0)
    pub precio: Decimal,
    /// Stock disponible (CHECK: stock >= 0)
    pub stock: i32,
    /// Categoría del producto
    pub categoria: Option<String>,
    /// Código SKU único
    pub sku: Option<String>,
    /// Estado activo/inactivo
    pub estado: bool,
    /// Fecha de creación
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Fecha de última actualización
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
