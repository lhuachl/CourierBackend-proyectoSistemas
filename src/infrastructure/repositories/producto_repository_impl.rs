use crate::domain::entities::Producto;
use crate::domain::repositories::ProductoRepository;
use crate::shared::error::{AppError, AppResult};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

/// Implementación del repositorio de productos con SQLx
pub struct ProductoRepositoryImpl {
    pool: PgPool,
}

impl ProductoRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ProductoRepository for ProductoRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Producto>> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"
            SELECT id_producto, nombre_producto, descripcion, precio, stock,
                   categoria, sku, estado, created_at, updated_at
            FROM productos
            WHERE id_producto = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(producto)
    }

    async fn find_by_sku(&self, sku: &str) -> AppResult<Option<Producto>> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"
            SELECT id_producto, nombre_producto, descripcion, precio, stock,
                   categoria, sku, estado, created_at, updated_at
            FROM productos
            WHERE sku = $1
            "#,
        )
        .bind(sku)
        .fetch_optional(&self.pool)
        .await?;

        Ok(producto)
    }

    async fn find_activos(&self) -> AppResult<Vec<Producto>> {
        let productos = sqlx::query_as::<_, Producto>(
            r#"
            SELECT id_producto, nombre_producto, descripcion, precio, stock,
                   categoria, sku, estado, created_at, updated_at
            FROM productos
            WHERE estado = true
            ORDER BY nombre_producto ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(productos)
    }

    async fn find_all(&self) -> AppResult<Vec<Producto>> {
        let productos = sqlx::query_as::<_, Producto>(
            r#"
            SELECT id_producto, nombre_producto, descripcion, precio, stock,
                   categoria, sku, estado, created_at, updated_at
            FROM productos
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(productos)
    }

    async fn find_by_categoria(&self, categoria: &str) -> AppResult<Vec<Producto>> {
        let productos = sqlx::query_as::<_, Producto>(
            r#"
            SELECT id_producto, nombre_producto, descripcion, precio, stock,
                   categoria, sku, estado, created_at, updated_at
            FROM productos
            WHERE categoria = $1 AND estado = true
            ORDER BY nombre_producto ASC
            "#,
        )
        .bind(categoria)
        .fetch_all(&self.pool)
        .await?;

        Ok(productos)
    }

    async fn search(&self, query: &str) -> AppResult<Vec<Producto>> {
        let search_pattern = format!("%{}%", query);
        let productos = sqlx::query_as::<_, Producto>(
            r#"
            SELECT id_producto, nombre_producto, descripcion, precio, stock,
                   categoria, sku, estado, created_at, updated_at
            FROM productos
            WHERE (nombre_producto ILIKE $1 OR sku ILIKE $1 OR descripcion ILIKE $1)
              AND estado = true
            ORDER BY nombre_producto ASC
            "#,
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(productos)
    }

    async fn create(
        &self,
        nombre: &str,
        descripcion: Option<&str>,
        precio: Decimal,
        stock: i32,
        categoria: Option<&str>,
        sku: Option<&str>,
    ) -> AppResult<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"
            INSERT INTO productos (nombre_producto, descripcion, precio, stock, categoria, sku)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id_producto, nombre_producto, descripcion, precio, stock,
                      categoria, sku, estado, created_at, updated_at
            "#,
        )
        .bind(nombre)
        .bind(descripcion)
        .bind(precio)
        .bind(stock)
        .bind(categoria)
        .bind(sku)
        .fetch_one(&self.pool)
        .await?;

        Ok(producto)
    }

    async fn update(
        &self,
        id: Uuid,
        nombre: Option<&str>,
        descripcion: Option<&str>,
        precio: Option<Decimal>,
        categoria: Option<&str>,
        sku: Option<&str>,
    ) -> AppResult<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"
            UPDATE productos
            SET nombre_producto = COALESCE($2, nombre_producto),
                descripcion = COALESCE($3, descripcion),
                precio = COALESCE($4, precio),
                categoria = COALESCE($5, categoria),
                sku = COALESCE($6, sku),
                updated_at = NOW()
            WHERE id_producto = $1
            RETURNING id_producto, nombre_producto, descripcion, precio, stock,
                      categoria, sku, estado, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(nombre)
        .bind(descripcion)
        .bind(precio)
        .bind(categoria)
        .bind(sku)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Producto {} no encontrado", id)),
            _ => AppError::Database(e),
        })?;

        Ok(producto)
    }

    async fn update_stock(&self, id: Uuid, cantidad: i32) -> AppResult<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"
            UPDATE productos
            SET stock = stock + $2,
                updated_at = NOW()
            WHERE id_producto = $1 AND (stock + $2) >= 0
            RETURNING id_producto, nombre_producto, descripcion, precio, stock,
                      categoria, sku, estado, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(cantidad)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::BadRequest(
                "Producto no encontrado o stock resultante sería negativo".to_string(),
            ),
            _ => AppError::Database(e),
        })?;

        Ok(producto)
    }

    async fn update_estado(&self, id: Uuid, estado: bool) -> AppResult<Producto> {
        let producto = sqlx::query_as::<_, Producto>(
            r#"
            UPDATE productos
            SET estado = $2,
                updated_at = NOW()
            WHERE id_producto = $1
            RETURNING id_producto, nombre_producto, descripcion, precio, stock,
                      categoria, sku, estado, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(estado)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Producto {} no encontrado", id)),
            _ => AppError::Database(e),
        })?;

        Ok(producto)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM productos WHERE id_producto = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Producto {} no encontrado", id)));
        }

        Ok(())
    }

    async fn exists_sku(&self, sku: &str) -> AppResult<bool> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM productos WHERE sku = $1",
        )
        .bind(sku)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(count,)| count > 0).unwrap_or(false))
    }
}
