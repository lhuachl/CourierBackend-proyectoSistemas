use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Pago {
    pub id_pago: Uuid,
    pub id_factura: Uuid,
    pub monto: f64,
    pub metodo_pago: String,
    pub estado: String,
    pub referencia_externa: Option<String>,
    pub fecha_pago: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
