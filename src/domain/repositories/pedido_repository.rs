use crate::domain::entities::Pedido;
use crate::shared::AppResult;
use uuid::Uuid;

/// Trait que define las operaciones de persistencia para Pedidos
/// Sigue el principio de Interface Segregation (ISP)
#[async_trait::async_trait]
pub trait PedidoRepository: Send + Sync {
    /// Buscar pedido por ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Pedido>>;
    
    /// Listar todos los pedidos de un perfil/usuario
    async fn find_by_perfil(&self, id_perfil: Uuid) -> AppResult<Vec<Pedido>>;
    
    /// Listar pedidos asignados a un transportista
    async fn find_by_transportista(&self, id_transportista: Uuid) -> AppResult<Vec<Pedido>>;
    
    /// Crear nuevo pedido
    async fn create(&self, pedido: &Pedido) -> AppResult<Pedido>;
    
    /// Actualizar estado del pedido
    async fn update_estado(&self, id: Uuid, estado: &str) -> AppResult<Pedido>;
    
    /// Asignar transportista al pedido
    async fn assign_transportista(&self, id: Uuid, id_transportista: Uuid) -> AppResult<Pedido>;
    
    /// Eliminar pedido (soft delete o cancelación)
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
