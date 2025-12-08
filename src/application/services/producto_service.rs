use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreateProductoDTO, UpdateProductoDTO, UpdateStockDTO, UpdateEstadoProductoDTO,
    ProductoResponseDTO, ProductosListResponseDTO,
};
use crate::domain::repositories::ProductoRepository;
use crate::shared::error::{AppError, AppResult};

/// Service que maneja la lógica de negocio de productos
pub struct ProductoService {
    repository: Arc<dyn ProductoRepository>,
}

impl ProductoService {
    pub fn new(repository: Arc<dyn ProductoRepository>) -> Self {
        Self { repository }
    }

    // ========================================================================
    // CONSULTAS
    // ========================================================================

    /// Lista todos los productos activos
    pub async fn list_productos(&self) -> AppResult<ProductosListResponseDTO> {
        let productos = self.repository.find_activos().await?;
        Ok(productos.into())
    }

    /// Lista todos los productos (incluye inactivos) - Admin
    pub async fn list_all_productos(&self) -> AppResult<ProductosListResponseDTO> {
        let productos = self.repository.find_all().await?;
        Ok(productos.into())
    }

    /// Obtiene un producto por ID
    pub async fn get_producto(&self, id: Uuid) -> AppResult<ProductoResponseDTO> {
        let producto = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", id)))?;

        Ok(producto.into())
    }

    /// Obtiene un producto por SKU
    pub async fn get_producto_by_sku(&self, sku: &str) -> AppResult<ProductoResponseDTO> {
        let producto = self
            .repository
            .find_by_sku(sku)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Producto con SKU {} no encontrado", sku)))?;

        Ok(producto.into())
    }

    /// Busca productos por categoría
    pub async fn get_by_categoria(&self, categoria: &str) -> AppResult<ProductosListResponseDTO> {
        let productos = self.repository.find_by_categoria(categoria).await?;
        Ok(productos.into())
    }

    /// Busca productos por nombre, SKU o descripción
    pub async fn search_productos(&self, query: &str) -> AppResult<ProductosListResponseDTO> {
        if query.trim().is_empty() {
            return Err(AppError::BadRequest("El término de búsqueda no puede estar vacío".into()));
        }
        
        let productos = self.repository.search(query).await?;
        Ok(productos.into())
    }

    // ========================================================================
    // MUTACIONES
    // ========================================================================

    /// Crea un nuevo producto
    pub async fn create_producto(&self, dto: CreateProductoDTO) -> AppResult<ProductoResponseDTO> {
        // Validar nombre
        if dto.nombre_producto.trim().is_empty() {
            return Err(AppError::BadRequest("El nombre del producto es requerido".into()));
        }

        // Validar precio
        if dto.precio <= 0.0 {
            return Err(AppError::BadRequest("El precio debe ser mayor a 0".into()));
        }

        // Validar stock inicial
        let stock = dto.stock.unwrap_or(0);
        if stock < 0 {
            return Err(AppError::BadRequest("El stock no puede ser negativo".into()));
        }

        // Validar SKU único si se proporciona
        if let Some(ref sku) = dto.sku {
            if !sku.is_empty() {
                if self.repository.exists_sku(sku).await? {
                    return Err(AppError::BadRequest(format!("Ya existe un producto con SKU {}", sku)));
                }
                
                // Validar formato SKU (alfanumérico con guiones)
                if !Self::validar_sku(sku) {
                    return Err(AppError::BadRequest(
                        "El SKU solo puede contener letras, números y guiones".into()
                    ));
                }
            }
        }

        // Convertir precio a Decimal
        let precio = Decimal::try_from(dto.precio)
            .map_err(|_| AppError::BadRequest("Precio inválido".into()))?;

        let producto = self
            .repository
            .create(
                dto.nombre_producto.trim(),
                dto.descripcion.as_deref(),
                precio,
                stock,
                dto.categoria.as_deref(),
                dto.sku.as_deref(),
            )
            .await?;

        tracing::info!("Producto creado: {} ({})", producto.nombre_producto, producto.id_producto);
        Ok(producto.into())
    }

    /// Actualiza un producto existente
    pub async fn update_producto(&self, id: Uuid, dto: UpdateProductoDTO) -> AppResult<ProductoResponseDTO> {
        // Verificar que existe
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", id)))?;

        // Validar precio si se actualiza
        let precio = if let Some(p) = dto.precio {
            if p <= 0.0 {
                return Err(AppError::BadRequest("El precio debe ser mayor a 0".into()));
            }
            Some(Decimal::try_from(p).map_err(|_| AppError::BadRequest("Precio inválido".into()))?)
        } else {
            None
        };

        // Validar SKU si se actualiza
        if let Some(ref sku) = dto.sku {
            if !sku.is_empty() {
                // Verificar que no esté en uso por otro producto
                if let Some(existing) = self.repository.find_by_sku(sku).await? {
                    if existing.id_producto != id {
                        return Err(AppError::BadRequest(format!("Ya existe un producto con SKU {}", sku)));
                    }
                }
                
                if !Self::validar_sku(sku) {
                    return Err(AppError::BadRequest(
                        "El SKU solo puede contener letras, números y guiones".into()
                    ));
                }
            }
        }

        let producto = self
            .repository
            .update(
                id,
                dto.nombre_producto.as_deref(),
                dto.descripcion.as_deref(),
                precio,
                dto.categoria.as_deref(),
                dto.sku.as_deref(),
            )
            .await?;

        tracing::info!("Producto actualizado: {}", id);
        Ok(producto.into())
    }

    /// Actualiza el stock de un producto
    pub async fn update_stock(&self, id: Uuid, dto: UpdateStockDTO) -> AppResult<ProductoResponseDTO> {
        // Verificar que existe
        let producto_actual = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", id)))?;

        // Verificar que el stock resultante no sea negativo
        let nuevo_stock = producto_actual.stock + dto.cantidad;
        if nuevo_stock < 0 {
            return Err(AppError::BadRequest(format!(
                "Stock insuficiente. Stock actual: {}, ajuste solicitado: {}",
                producto_actual.stock, dto.cantidad
            )));
        }

        let producto = self.repository.update_stock(id, dto.cantidad).await?;

        tracing::info!(
            "Stock actualizado para {}: {} -> {} (motivo: {:?})",
            id, producto_actual.stock, producto.stock, dto.motivo
        );
        Ok(producto.into())
    }

    /// Cambia el estado de un producto (activar/desactivar)
    pub async fn update_estado(&self, id: Uuid, dto: UpdateEstadoProductoDTO) -> AppResult<ProductoResponseDTO> {
        let producto = self.repository.update_estado(id, dto.estado).await?;

        tracing::info!(
            "Estado del producto {} cambiado a: {}",
            id, if dto.estado { "activo" } else { "inactivo" }
        );
        Ok(producto.into())
    }

    /// Elimina un producto (hard delete)
    pub async fn delete_producto(&self, id: Uuid) -> AppResult<()> {
        // Verificar que existe
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", id)))?;

        // TODO: Verificar que no esté asociado a pedidos activos
        
        self.repository.delete(id).await?;
        tracing::info!("Producto eliminado: {}", id);
        Ok(())
    }

    // ========================================================================
    // HELPERS PRIVADOS
    // ========================================================================

    /// Valida el formato de un SKU
    fn validar_sku(sku: &str) -> bool {
        !sku.is_empty() 
            && sku.len() <= 50 
            && sku.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}
