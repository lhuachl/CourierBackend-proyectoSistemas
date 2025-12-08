use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pedido {
    pub id_pedido: Uuid,
    pub numero_tracking: String,
    pub id_perfil: Uuid,
    pub id_transportista: Option<Uuid>,
    pub id_direccion_origen: Uuid,
    pub id_direccion_destino: Uuid,
    pub estado: String,
    pub fecha_entrega_estimada: Option<chrono::DateTime<chrono::Utc>>,
    pub fecha_entrega_real: Option<chrono::DateTime<chrono::Utc>>,
    pub monto_total: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
