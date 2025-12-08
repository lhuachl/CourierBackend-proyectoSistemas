use crate::domain::entities::PerfilCliente;
use uuid::Uuid;

/// Trait que define las operaciones del repositorio de perfiles de cliente
#[async_trait::async_trait]
pub trait PerfilClienteRepository: Send + Sync {
    /// Busca un perfil por su ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PerfilCliente>, sqlx::Error>;
    
    /// Busca un perfil por el ID del usuario
    async fn find_by_usuario(&self, id_usuario: Uuid) -> Result<Option<PerfilCliente>, sqlx::Error>;
    
    /// Obtiene todos los perfiles (admin)
    async fn find_all(&self) -> Result<Vec<PerfilCliente>, sqlx::Error>;
    
    /// Crea un nuevo perfil
    async fn create(&self, id_usuario: Uuid, documento: Option<&str>, telefono: Option<&str>) -> Result<PerfilCliente, sqlx::Error>;
    
    /// Actualiza un perfil existente
    async fn update(&self, id: Uuid, documento: Option<&str>, telefono: Option<&str>) -> Result<PerfilCliente, sqlx::Error>;
    
    /// Elimina un perfil (soft delete en el futuro)
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    
    /// Verifica si existe un perfil para el usuario
    async fn exists_for_usuario(&self, id_usuario: Uuid) -> Result<bool, sqlx::Error>;
    
    /// Busca perfil por documento de identidad
    async fn find_by_documento(&self, documento: &str) -> Result<Option<PerfilCliente>, sqlx::Error>;
}
