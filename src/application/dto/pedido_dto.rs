use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

use crate::domain::entities::Pedido;

/// DTO para crear un nuevo pedido
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePedidoDTO {
    pub id_direccion_origen: Uuid,
    pub id_direccion_destino: Uuid,
    pub monto_total: f64,
}

/// DTO para actualizar estado del pedido
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateEstadoPedidoDTO {
    /// Estado: pendiente, confirmado, en_transito, entregado, cancelado
    pub estado: String,
}

/// DTO para asignar transportista
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AsignarTransportistaDTO {
    pub id_transportista: Uuid,
}

/// DTO de respuesta para pedido
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PedidoResponseDTO {
    pub id_pedido: Uuid,
    pub numero_tracking: String,
    pub id_perfil: Uuid,
    pub id_transportista: Option<Uuid>,
    pub id_direccion_origen: Uuid,
    pub id_direccion_destino: Uuid,
    pub estado: String,
    pub fecha_entrega_estimada: Option<DateTime<Utc>>,
    pub fecha_entrega_real: Option<DateTime<Utc>>,
    pub monto_total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Conversión de Entity a DTO (DRY)
impl From<Pedido> for PedidoResponseDTO {
    fn from(p: Pedido) -> Self {
        Self {
            id_pedido: p.id_pedido,
            numero_tracking: p.numero_tracking,
            id_perfil: p.id_perfil,
            id_transportista: p.id_transportista,
            id_direccion_origen: p.id_direccion_origen,
            id_direccion_destino: p.id_direccion_destino,
            estado: p.estado,
            fecha_entrega_estimada: p.fecha_entrega_estimada,
            fecha_entrega_real: p.fecha_entrega_real,
            monto_total: p.monto_total,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

/// Lista de pedidos
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PedidosListResponseDTO {
    pub pedidos: Vec<PedidoResponseDTO>,
    pub total: usize,
}
