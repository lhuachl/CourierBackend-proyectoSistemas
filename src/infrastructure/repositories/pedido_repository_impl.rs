use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;

use crate::domain::entities::Pedido;
use crate::domain::repositories::PedidoRepository;
use crate::shared::{AppError, AppResult};

/// Implementación concreta del repositorio de pedidos usando SQLx
/// Sigue SRP: solo se encarga de persistencia
pub struct PedidoRepositoryImpl {
    pool: PgPool,
}

impl PedidoRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PedidoRepository for PedidoRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Pedido>> {
        let pedido = sqlx::query_as::<_, Pedido>(
            r#"
            SELECT id_pedido, numero_tracking, id_perfil, id_transportista,
                   id_direccion_origen, id_direccion_destino, estado,
                   fecha_entrega_estimada, fecha_entrega_real, monto_total,
                   created_at, updated_at
            FROM pedidos
            WHERE id_pedido = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(pedido)
    }

    async fn find_by_perfil(&self, id_perfil: Uuid) -> AppResult<Vec<Pedido>> {
        let pedidos = sqlx::query_as::<_, Pedido>(
            r#"
            SELECT id_pedido, numero_tracking, id_perfil, id_transportista,
                   id_direccion_origen, id_direccion_destino, estado,
                   fecha_entrega_estimada, fecha_entrega_real, monto_total,
                   created_at, updated_at
            FROM pedidos
            WHERE id_perfil = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(id_perfil)
        .fetch_all(&self.pool)
        .await?;

        Ok(pedidos)
    }

    async fn find_by_transportista(&self, id_transportista: Uuid) -> AppResult<Vec<Pedido>> {
        let pedidos = sqlx::query_as::<_, Pedido>(
            r#"
            SELECT id_pedido, numero_tracking, id_perfil, id_transportista,
                   id_direccion_origen, id_direccion_destino, estado,
                   fecha_entrega_estimada, fecha_entrega_real, monto_total,
                   created_at, updated_at
            FROM pedidos
            WHERE id_transportista = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(id_transportista)
        .fetch_all(&self.pool)
        .await?;

        Ok(pedidos)
    }

    async fn create(&self, pedido: &Pedido) -> AppResult<Pedido> {
        let created = sqlx::query_as::<_, Pedido>(
            r#"
            INSERT INTO pedidos (
                id_perfil, id_direccion_origen, id_direccion_destino,
                estado, monto_total
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id_pedido, numero_tracking, id_perfil, id_transportista,
                      id_direccion_origen, id_direccion_destino, estado,
                      fecha_entrega_estimada, fecha_entrega_real, monto_total,
                      created_at, updated_at
            "#
        )
        .bind(&pedido.id_perfil)
        .bind(&pedido.id_direccion_origen)
        .bind(&pedido.id_direccion_destino)
        .bind(&pedido.estado)
        .bind(&pedido.monto_total)
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    async fn update_estado(&self, id: Uuid, estado: &str) -> AppResult<Pedido> {
        let updated = sqlx::query_as::<_, Pedido>(
            r#"
            UPDATE pedidos
            SET estado = $2, updated_at = NOW()
            WHERE id_pedido = $1
            RETURNING id_pedido, numero_tracking, id_perfil, id_transportista,
                      id_direccion_origen, id_direccion_destino, estado,
                      fecha_entrega_estimada, fecha_entrega_real, monto_total,
                      created_at, updated_at
            "#
        )
        .bind(id)
        .bind(estado)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Pedido {} no encontrado", id)))?;

        Ok(updated)
    }

    async fn assign_transportista(&self, id: Uuid, id_transportista: Uuid) -> AppResult<Pedido> {
        let updated = sqlx::query_as::<_, Pedido>(
            r#"
            UPDATE pedidos
            SET id_transportista = $2, updated_at = NOW()
            WHERE id_pedido = $1
            RETURNING id_pedido, numero_tracking, id_perfil, id_transportista,
                      id_direccion_origen, id_direccion_destino, estado,
                      fecha_entrega_estimada, fecha_entrega_real, monto_total,
                      created_at, updated_at
            "#
        )
        .bind(id)
        .bind(id_transportista)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Pedido {} no encontrado", id)))?;

        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE pedidos
            SET estado = 'cancelado', updated_at = NOW()
            WHERE id_pedido = $1
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Pedido {} no encontrado", id)));
        }

        Ok(())
    }
}
