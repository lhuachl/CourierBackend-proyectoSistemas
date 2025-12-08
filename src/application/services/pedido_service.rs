use std::sync::Arc;
use uuid::Uuid;

use crate::application::dto::{
    CreatePedidoDTO, PedidoResponseDTO, PedidosListResponseDTO,
    UpdateEstadoPedidoDTO, AsignarTransportistaDTO,
};
use crate::domain::entities::Pedido;
use crate::domain::repositories::PedidoRepository;
use crate::shared::{AppError, AppResult};

/// Estados válidos para transiciones
const ESTADOS_VALIDOS: [&str; 5] = ["pendiente", "confirmado", "en_transito", "entregado", "cancelado"];

/// Service de pedidos - contiene la lógica de negocio
/// Sigue SRP: solo lógica de negocio, delega persistencia al repository
pub struct PedidoService {
    repository: Arc<dyn PedidoRepository>,
}

impl PedidoService {
    pub fn new(repository: Arc<dyn PedidoRepository>) -> Self {
        Self { repository }
    }

    /// Obtener pedido por ID
    pub async fn get_by_id(&self, id: Uuid) -> AppResult<PedidoResponseDTO> {
        let pedido = self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Pedido {} no encontrado", id)))?;

        Ok(pedido.into())
    }

    /// Listar pedidos de un usuario
    pub async fn list_by_user(&self, id_perfil: Uuid) -> AppResult<PedidosListResponseDTO> {
        let pedidos = self.repository.find_by_perfil(id_perfil).await?;
        let total = pedidos.len();

        Ok(PedidosListResponseDTO {
            pedidos: pedidos.into_iter().map(|p| p.into()).collect(),
            total,
        })
    }

    /// Listar pedidos de un transportista
    pub async fn list_by_transportista(&self, id_transportista: Uuid) -> AppResult<PedidosListResponseDTO> {
        let pedidos = self.repository.find_by_transportista(id_transportista).await?;
        let total = pedidos.len();

        Ok(PedidosListResponseDTO {
            pedidos: pedidos.into_iter().map(|p| p.into()).collect(),
            total,
        })
    }

    /// Crear nuevo pedido
    pub async fn create(&self, id_perfil: Uuid, dto: CreatePedidoDTO) -> AppResult<PedidoResponseDTO> {
        // Validar monto
        if dto.monto_total <= 0.0 {
            return Err(AppError::BadRequest("El monto total debe ser mayor a 0".to_string()));
        }

        // Crear entidad de pedido
        let pedido = Pedido {
            id_pedido: Uuid::new_v4(),
            numero_tracking: String::new(), // Se genera en la BD
            id_perfil,
            id_transportista: None,
            id_direccion_origen: dto.id_direccion_origen,
            id_direccion_destino: dto.id_direccion_destino,
            estado: "pendiente".to_string(),
            fecha_entrega_estimada: None,
            fecha_entrega_real: None,
            monto_total: dto.monto_total,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let created = self.repository.create(&pedido).await?;
        Ok(created.into())
    }

    /// Actualizar estado del pedido
    pub async fn update_estado(&self, id: Uuid, dto: UpdateEstadoPedidoDTO) -> AppResult<PedidoResponseDTO> {
        // Validar estado
        if !ESTADOS_VALIDOS.contains(&dto.estado.as_str()) {
            return Err(AppError::BadRequest(format!(
                "Estado inválido. Estados válidos: {:?}",
                ESTADOS_VALIDOS
            )));
        }

        // Obtener pedido actual para validar transición
        let pedido_actual = self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Pedido {} no encontrado", id)))?;

        // Validar transición de estado
        self.validar_transicion_estado(&pedido_actual.estado, &dto.estado)?;

        let updated = self.repository.update_estado(id, &dto.estado).await?;
        Ok(updated.into())
    }

    /// Asignar transportista al pedido
    pub async fn assign_transportista(&self, id: Uuid, dto: AsignarTransportistaDTO) -> AppResult<PedidoResponseDTO> {
        // Verificar que el pedido existe
        let pedido = self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Pedido {} no encontrado", id)))?;

        // Solo se puede asignar en estados pendiente o confirmado
        if pedido.estado != "pendiente" && pedido.estado != "confirmado" {
            return Err(AppError::BadRequest(
                "Solo se puede asignar transportista a pedidos pendientes o confirmados".to_string()
            ));
        }

        let updated = self.repository.assign_transportista(id, dto.id_transportista).await?;
        Ok(updated.into())
    }

    /// Cancelar pedido
    pub async fn cancel(&self, id: Uuid) -> AppResult<()> {
        // Verificar que el pedido existe y puede ser cancelado
        let pedido = self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Pedido {} no encontrado", id)))?;

        // No se puede cancelar si ya está entregado
        if pedido.estado == "entregado" {
            return Err(AppError::BadRequest(
                "No se puede cancelar un pedido ya entregado".to_string()
            ));
        }

        self.repository.delete(id).await
    }

    /// Validar transición de estado (regla de negocio)
    fn validar_transicion_estado(&self, estado_actual: &str, nuevo_estado: &str) -> AppResult<()> {
        let transiciones_validas: Vec<(&str, Vec<&str>)> = vec![
            ("pendiente", vec!["confirmado", "cancelado"]),
            ("confirmado", vec!["en_transito", "cancelado"]),
            ("en_transito", vec!["entregado", "cancelado"]),
            ("entregado", vec![]),
            ("cancelado", vec![]),
        ];

        let transicion_permitida = transiciones_validas
            .iter()
            .find(|(estado, _)| *estado == estado_actual)
            .map(|(_, permitidos)| permitidos.contains(&nuevo_estado))
            .unwrap_or(false);

        if !transicion_permitida {
            return Err(AppError::BadRequest(format!(
                "Transición de estado no permitida: {} → {}",
                estado_actual, nuevo_estado
            )));
        }

        Ok(())
    }
}
