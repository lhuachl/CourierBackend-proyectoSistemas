use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Factura {
    pub id_factura: Uuid,
    pub id_pedido: Uuid,
    pub numero_factura: String,
    pub subtotal: f64,
    pub impuestos: f64,
    pub total: Option<f64>,
    pub estado: String,
    pub fecha_emision: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
