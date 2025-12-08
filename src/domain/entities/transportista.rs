use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transportista {
    pub id_transportista: Uuid,
    pub id_usuario: Uuid,
    pub tipo_vehiculo: String,
    pub placa_vehiculo: String,
    pub capacidad_carga: f64,
    pub estado: String,
    pub id_zona_asignada: Option<Uuid>,
    pub calificacion_promedio: Option<f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
