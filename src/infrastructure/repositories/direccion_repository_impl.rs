use crate::domain::entities::Direccion;
use crate::domain::repositories::DireccionRepository;
use crate::shared::error::{AppError, AppResult};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

/// Implementación del repositorio de direcciones con SQLx
pub struct DireccionRepositoryImpl {
    pool: PgPool,
}

impl DireccionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl DireccionRepository for DireccionRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Direccion>> {
        let direccion = sqlx::query_as::<_, Direccion>(
            r#"
            SELECT id_direccion, id_perfil, tipo, calle, ciudad, 
                   referencias_adicionales, pais, latitud, longitud,
                   es_predeterminada, activo, created_at, updated_at
            FROM direcciones
            WHERE id_direccion = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(direccion)
    }

    async fn find_by_perfil(&self, id_perfil: Uuid) -> AppResult<Vec<Direccion>> {
        let direcciones = sqlx::query_as::<_, Direccion>(
            r#"
            SELECT id_direccion, id_perfil, tipo, calle, ciudad, 
                   referencias_adicionales, pais, latitud, longitud,
                   es_predeterminada, activo, created_at, updated_at
            FROM direcciones
            WHERE id_perfil = $1 AND activo = true
            ORDER BY es_predeterminada DESC, created_at DESC
            "#,
        )
        .bind(id_perfil)
        .fetch_all(&self.pool)
        .await?;

        Ok(direcciones)
    }

    async fn find_all_by_perfil(&self, id_perfil: Uuid) -> AppResult<Vec<Direccion>> {
        let direcciones = sqlx::query_as::<_, Direccion>(
            r#"
            SELECT id_direccion, id_perfil, tipo, calle, ciudad, 
                   referencias_adicionales, pais, latitud, longitud,
                   es_predeterminada, activo, created_at, updated_at
            FROM direcciones
            WHERE id_perfil = $1
            ORDER BY activo DESC, es_predeterminada DESC, created_at DESC
            "#,
        )
        .bind(id_perfil)
        .fetch_all(&self.pool)
        .await?;

        Ok(direcciones)
    }

    async fn find_almacenes(&self) -> AppResult<Vec<Direccion>> {
        let direcciones = sqlx::query_as::<_, Direccion>(
            r#"
            SELECT id_direccion, id_perfil, tipo, calle, ciudad, 
                   referencias_adicionales, pais, latitud, longitud,
                   es_predeterminada, activo, created_at, updated_at
            FROM direcciones
            WHERE tipo = 'almacen' AND activo = true
            ORDER BY ciudad ASC, calle ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(direcciones)
    }

    async fn find_all_almacenes(&self) -> AppResult<Vec<Direccion>> {
        let direcciones = sqlx::query_as::<_, Direccion>(
            r#"
            SELECT id_direccion, id_perfil, tipo, calle, ciudad, 
                   referencias_adicionales, pais, latitud, longitud,
                   es_predeterminada, activo, created_at, updated_at
            FROM direcciones
            WHERE tipo = 'almacen'
            ORDER BY activo DESC, ciudad ASC, calle ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(direcciones)
    }

    async fn find_predeterminada(&self, id_perfil: Uuid) -> AppResult<Option<Direccion>> {
        let direccion = sqlx::query_as::<_, Direccion>(
            r#"
            SELECT id_direccion, id_perfil, tipo, calle, ciudad, 
                   referencias_adicionales, pais, latitud, longitud,
                   es_predeterminada, activo, created_at, updated_at
            FROM direcciones
            WHERE id_perfil = $1 AND es_predeterminada = true AND activo = true
            "#,
        )
        .bind(id_perfil)
        .fetch_optional(&self.pool)
        .await?;

        Ok(direccion)
    }

    async fn create(
        &self,
        id_perfil: Option<Uuid>,
        tipo: &str,
        calle: &str,
        ciudad: &str,
        pais: &str,
        referencias: Option<&str>,
        latitud: Decimal,
        longitud: Decimal,
        es_predeterminada: bool,
    ) -> AppResult<Direccion> {
        let direccion = sqlx::query_as::<_, Direccion>(
            r#"
            INSERT INTO direcciones (id_perfil, tipo, calle, ciudad, pais, 
                                     referencias_adicionales, latitud, longitud, 
                                     es_predeterminada, activo)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, true)
            RETURNING id_direccion, id_perfil, tipo, calle, ciudad, 
                      referencias_adicionales, pais, latitud, longitud,
                      es_predeterminada, activo, created_at, updated_at
            "#,
        )
        .bind(id_perfil)
        .bind(tipo)
        .bind(calle)
        .bind(ciudad)
        .bind(pais)
        .bind(referencias)
        .bind(latitud)
        .bind(longitud)
        .bind(es_predeterminada)
        .fetch_one(&self.pool)
        .await?;

        Ok(direccion)
    }

    async fn update(
        &self,
        id: Uuid,
        calle: Option<&str>,
        ciudad: Option<&str>,
        pais: Option<&str>,
        referencias: Option<&str>,
        latitud: Option<Decimal>,
        longitud: Option<Decimal>,
    ) -> AppResult<Direccion> {
        let direccion = sqlx::query_as::<_, Direccion>(
            r#"
            UPDATE direcciones
            SET calle = COALESCE($2, calle),
                ciudad = COALESCE($3, ciudad),
                pais = COALESCE($4, pais),
                referencias_adicionales = COALESCE($5, referencias_adicionales),
                latitud = COALESCE($6, latitud),
                longitud = COALESCE($7, longitud),
                updated_at = NOW()
            WHERE id_direccion = $1
            RETURNING id_direccion, id_perfil, tipo, calle, ciudad, 
                      referencias_adicionales, pais, latitud, longitud,
                      es_predeterminada, activo, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(calle)
        .bind(ciudad)
        .bind(pais)
        .bind(referencias)
        .bind(latitud)
        .bind(longitud)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Dirección {} no encontrada", id)),
            _ => AppError::Database(e),
        })?;

        Ok(direccion)
    }

    async fn set_predeterminada(&self, id: Uuid, id_perfil: Uuid) -> AppResult<Direccion> {
        // Primero quitar el flag de todas las direcciones del perfil
        sqlx::query(
            r#"
            UPDATE direcciones
            SET es_predeterminada = false, updated_at = NOW()
            WHERE id_perfil = $1 AND es_predeterminada = true
            "#,
        )
        .bind(id_perfil)
        .execute(&self.pool)
        .await?;

        // Luego establecer la nueva predeterminada
        let direccion = sqlx::query_as::<_, Direccion>(
            r#"
            UPDATE direcciones
            SET es_predeterminada = true, updated_at = NOW()
            WHERE id_direccion = $1
            RETURNING id_direccion, id_perfil, tipo, calle, ciudad, 
                      referencias_adicionales, pais, latitud, longitud,
                      es_predeterminada, activo, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Dirección {} no encontrada", id)),
            _ => AppError::Database(e),
        })?;

        Ok(direccion)
    }

    async fn deactivate(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE direcciones
            SET activo = false, es_predeterminada = false, updated_at = NOW()
            WHERE id_direccion = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Dirección {} no encontrada", id)));
        }

        Ok(())
    }

    async fn activate(&self, id: Uuid) -> AppResult<Direccion> {
        let direccion = sqlx::query_as::<_, Direccion>(
            r#"
            UPDATE direcciones
            SET activo = true, updated_at = NOW()
            WHERE id_direccion = $1
            RETURNING id_direccion, id_perfil, tipo, calle, ciudad, 
                      referencias_adicionales, pais, latitud, longitud,
                      es_predeterminada, activo, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Dirección {} no encontrada", id)),
            _ => AppError::Database(e),
        })?;

        Ok(direccion)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM direcciones WHERE id_direccion = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Dirección {} no encontrada", id)));
        }

        Ok(())
    }

    async fn belongs_to_perfil(&self, id: Uuid, id_perfil: Uuid) -> AppResult<bool> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM direcciones WHERE id_direccion = $1 AND id_perfil = $2",
        )
        .bind(id)
        .bind(id_perfil)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(count,)| count > 0).unwrap_or(false))
    }

    async fn count_by_perfil(&self, id_perfil: Uuid) -> AppResult<i64> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM direcciones WHERE id_perfil = $1 AND activo = true",
        )
        .bind(id_perfil)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }
}
