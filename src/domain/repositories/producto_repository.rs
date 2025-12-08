use crate::domain::entities::Producto;
use crate::shared::error::AppResult;
use uuid::Uuid;

/// Trait que define las operaciones del repositorio de productos
#[async_trait::async_trait]
pub trait ProductoRepository: Send + Sync {
    /// Busca un producto por su ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Producto>>;
    
    /// Busca un producto por su SKU
    async fn find_by_sku(&self, sku: &str) -> AppResult<Option<Producto>>;
    
    /// Lista todos los productos activos
    async fn find_activos(&self) -> AppResult<Vec<Producto>>;
    
    /// Lista todos los productos (incluye inactivos)
    async fn find_all(&self) -> AppResult<Vec<Producto>>;
    
    /// Busca productos por categoría
    async fn find_by_categoria(&self, categoria: &str) -> AppResult<Vec<Producto>>;
    
    /// Busca productos por nombre (ILIKE)
    async fn search(&self, query: &str) -> AppResult<Vec<Producto>>;
    
    /// Crea un nuevo producto
    async fn create(
        &self,
        nombre: &str,
        descripcion: Option<&str>,
        precio: rust_decimal::Decimal,
        stock: i32,
        categoria: Option<&str>,
        sku: Option<&str>,
    ) -> AppResult<Producto>;
    
    /// Actualiza un producto existente
    async fn update(
        &self,
        id: Uuid,
        nombre: Option<&str>,
        descripcion: Option<&str>,
        precio: Option<rust_decimal::Decimal>,
        categoria: Option<&str>,
        sku: Option<&str>,
    ) -> AppResult<Producto>;
    
    /// Actualiza el stock de un producto
    async fn update_stock(&self, id: Uuid, cantidad: i32) -> AppResult<Producto>;
    
    /// Cambia el estado (activo/inactivo) de un producto
    async fn update_estado(&self, id: Uuid, estado: bool) -> AppResult<Producto>;
    
    /// Elimina un producto (hard delete)
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// Verifica si existe un producto con el SKU dado
    async fn exists_sku(&self, sku: &str) -> AppResult<bool>;
}
