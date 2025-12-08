use crate::domain::entities::PerfilCliente;
use crate::domain::repositories::PerfilClienteRepository;
use sqlx::PgPool;
use uuid::Uuid;

/// Implementación del repositorio de perfiles de cliente con SQLx
pub struct PerfilClienteRepositoryImpl {
    pool: PgPool,
}

impl PerfilClienteRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PerfilClienteRepository for PerfilClienteRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PerfilCliente>, sqlx::Error> {
        sqlx::query_as::<_, PerfilCliente>(
            r#"
            SELECT id_perfil, id_usuario, documento_identidad, telefono, created_at, updated_at
            FROM perfiles_cliente
            WHERE id_perfil = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_usuario(&self, id_usuario: Uuid) -> Result<Option<PerfilCliente>, sqlx::Error> {
        sqlx::query_as::<_, PerfilCliente>(
            r#"
            SELECT id_perfil, id_usuario, documento_identidad, telefono, created_at, updated_at
            FROM perfiles_cliente
            WHERE id_usuario = $1
            "#,
        )
        .bind(id_usuario)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<PerfilCliente>, sqlx::Error> {
        sqlx::query_as::<_, PerfilCliente>(
            r#"
            SELECT id_perfil, id_usuario, documento_identidad, telefono, created_at, updated_at
            FROM perfiles_cliente
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn create(
        &self,
        id_usuario: Uuid,
        documento: Option<&str>,
        telefono: Option<&str>,
    ) -> Result<PerfilCliente, sqlx::Error> {
        sqlx::query_as::<_, PerfilCliente>(
            r#"
            INSERT INTO perfiles_cliente (id_usuario, documento_identidad, telefono)
            VALUES ($1, $2, $3)
            RETURNING id_perfil, id_usuario, documento_identidad, telefono, created_at, updated_at
            "#,
        )
        .bind(id_usuario)
        .bind(documento)
        .bind(telefono)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(
        &self,
        id: Uuid,
        documento: Option<&str>,
        telefono: Option<&str>,
    ) -> Result<PerfilCliente, sqlx::Error> {
        sqlx::query_as::<_, PerfilCliente>(
            r#"
            UPDATE perfiles_cliente
            SET documento_identidad = COALESCE($2, documento_identidad),
                telefono = COALESCE($3, telefono),
                updated_at = NOW()
            WHERE id_perfil = $1
            RETURNING id_perfil, id_usuario, documento_identidad, telefono, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(documento)
        .bind(telefono)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM perfiles_cliente WHERE id_perfil = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn exists_for_usuario(&self, id_usuario: Uuid) -> Result<bool, sqlx::Error> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM perfiles_cliente WHERE id_usuario = $1",
        )
        .bind(id_usuario)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(count,)| count > 0).unwrap_or(false))
    }

    async fn find_by_documento(&self, documento: &str) -> Result<Option<PerfilCliente>, sqlx::Error> {
        sqlx::query_as::<_, PerfilCliente>(
            r#"
            SELECT id_perfil, id_usuario, documento_identidad, telefono, created_at, updated_at
            FROM perfiles_cliente
            WHERE documento_identidad = $1
            "#,
        )
        .bind(documento)
        .fetch_optional(&self.pool)
        .await
    }
}
