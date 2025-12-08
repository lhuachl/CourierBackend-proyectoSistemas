use crate::domain::entities::Direccion;
use crate::shared::error::AppResult;
use rust_decimal::Decimal;
use uuid::Uuid;

/// Trait que define las operaciones del repositorio de direcciones
#[async_trait::async_trait]
pub trait DireccionRepository: Send + Sync {
    /// Busca una dirección por su ID
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Direccion>>;
    
    /// Busca todas las direcciones de un perfil (solo activas)
    async fn find_by_perfil(&self, id_perfil: Uuid) -> AppResult<Vec<Direccion>>;
    
    /// Busca todas las direcciones de un perfil (incluye inactivas)
    async fn find_all_by_perfil(&self, id_perfil: Uuid) -> AppResult<Vec<Direccion>>;
    
    /// Busca todas las direcciones de tipo almacén (activas)
    async fn find_almacenes(&self) -> AppResult<Vec<Direccion>>;
    
    /// Busca todas las direcciones de tipo almacén (incluye inactivas) - Admin
    async fn find_all_almacenes(&self) -> AppResult<Vec<Direccion>>;
    
    /// Obtiene la dirección predeterminada de un perfil
    async fn find_predeterminada(&self, id_perfil: Uuid) -> AppResult<Option<Direccion>>;
    
    /// Crea una nueva dirección
    async fn create(
        &self,
        id_perfil: Option<Uuid>,
        tipo: &str,
        calle: &str,
        ciudad: &str,
        pais: &str,
        referencias: Option<&str>,
        latitud: Decimal,
        longitud: Decimal,
        es_predeterminada: bool,
    ) -> AppResult<Direccion>;
    
    /// Actualiza una dirección existente
    async fn update(
        &self,
        id: Uuid,
        calle: Option<&str>,
        ciudad: Option<&str>,
        pais: Option<&str>,
        referencias: Option<&str>,
        latitud: Option<Decimal>,
        longitud: Option<Decimal>,
    ) -> AppResult<Direccion>;
    
    /// Establece una dirección como predeterminada (y quita el flag de las demás)
    async fn set_predeterminada(&self, id: Uuid, id_perfil: Uuid) -> AppResult<Direccion>;
    
    /// Desactiva una dirección (soft delete)
    async fn deactivate(&self, id: Uuid) -> AppResult<()>;
    
    /// Reactiva una dirección
    async fn activate(&self, id: Uuid) -> AppResult<Direccion>;
    
    /// Elimina una dirección permanentemente (hard delete)
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    
    /// Verifica si una dirección pertenece a un perfil
    async fn belongs_to_perfil(&self, id: Uuid, id_perfil: Uuid) -> AppResult<bool>;
    
    /// Cuenta las direcciones activas de un perfil
    async fn count_by_perfil(&self, id_perfil: Uuid) -> AppResult<i64>;
}
