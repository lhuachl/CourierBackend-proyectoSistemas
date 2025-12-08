use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Entidad PerfilCliente - Datos extendidos de un cliente
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PerfilCliente {
    pub id_perfil: Uuid,
    pub id_usuario: Uuid,
    pub documento_identidad: Option<String>,
    pub telefono: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
