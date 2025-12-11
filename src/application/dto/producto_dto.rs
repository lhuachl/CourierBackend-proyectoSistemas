use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::Producto;

// ============================================================================
// REQUEST DTOs
// ============================================================================

/// DTO para crear un nuevo producto
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateProductoDTO {
    /// Nombre del producto (requerido)
    #[schema(example = "Laptop Dell XPS 15")]
    pub nombre_producto: String,
    
    /// Descripción detallada del producto
    #[schema(example = "Laptop de alta gama con procesador Intel i7")]
    pub descripcion: Option<String>,
    
    /// Precio unitario (debe ser > 0)
    #[schema(example = 1299.99)]
    pub precio: f64,
    
    /// Stock inicial (debe ser >= 0)
    #[schema(example = 50)]
    pub stock: Option<i32>,
    
    /// Categoría del producto
    #[schema(example = "Electrónicos")]
    pub categoria: Option<String>,
    
    /// Código SKU único
    #[schema(example = "DELL-XPS15-2024")]
    pub sku: Option<String>,
}

/// DTO para actualizar un producto
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateProductoDTO {
    /// Nombre del producto
    #[schema(example = "Laptop Dell XPS 15 Pro")]
    pub nombre_producto: Option<String>,
    
    /// Descripción del producto
    pub descripcion: Option<String>,
    
    /// Nuevo precio
    #[schema(example = 1499.99)]
    pub precio: Option<f64>,
    
    /// Categoría
    pub categoria: Option<String>,
    
    /// Código SKU
    pub sku: Option<String>,
}

/// DTO para actualizar stock
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateStockDTO {
    /// Cantidad a agregar (positivo) o restar (negativo)
    #[schema(example = 10)]
    pub cantidad: i32,
    
    /// Motivo del ajuste
    #[schema(example = "Recepción de inventario")]
    pub motivo: Option<String>,
}

/// DTO para cambiar estado del producto
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateEstadoProductoDTO {
    /// Estado activo (true) o inactivo (false)
    #[schema(example = true)]
    pub estado: bool,
}

// ============================================================================
// RESPONSE DTOs
// ============================================================================

/// DTO de respuesta para un producto
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProductoResponseDTO {
    /// ID único del producto
    pub id_producto: Uuid,
    
    /// Nombre del producto
    pub nombre_producto: String,
    
    /// Descripción del producto
    pub descripcion: Option<String>,
    
    /// Precio unitario
    #[schema(example = 1299.99)]
    pub precio: f64,
    
    /// Stock disponible
    pub stock: i32,
    
    /// Categoría
    pub categoria: Option<String>,
    
    /// Código SKU
    pub sku: Option<String>,
    
    /// Estado activo/inactivo
    pub estado: bool,
    
    /// Fecha de creación
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Fecha de última actualización
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de respuesta para lista de productos
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProductosListResponseDTO {
    /// Lista de productos
    pub productos: Vec<ProductoResponseDTO>,
    
    /// Total de productos
    pub total: usize,
}

// ============================================================================
// CONVERSIONES (From impls)
// ============================================================================

impl From<Producto> for ProductoResponseDTO {
    fn from(p: Producto) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id_producto: p.id_producto,
            nombre_producto: p.nombre_producto,
            descripcion: p.descripcion,
            precio: p.precio.to_f64().unwrap_or(0.0),
            stock: p.stock,
            categoria: p.categoria,
            sku: p.sku,
            estado: p.estado,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

impl From<Vec<Producto>> for ProductosListResponseDTO {
    fn from(productos: Vec<Producto>) -> Self {
        let total = productos.len();
        Self {
            productos: productos.into_iter().map(Into::into).collect(),
            total,
        }
    }
}
