use crate::domain::entities::User;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    /// Obtiene un usuario por ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    
    /// Obtiene un usuario por email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    
    /// Obtiene todos los usuarios
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
    
    /// Obtiene todos los usuarios con paginación
    async fn find_all_paginated(&self, limit: i64, offset: i64) -> Result<(Vec<User>, i64), sqlx::Error>;
    
    /// Crea un nuevo usuario
    async fn create(&self, user: &User) -> Result<User, sqlx::Error>;
    
    /// Actualiza un usuario
    async fn update(&self, user: &User) -> Result<User, sqlx::Error>;
    
    /// Actualiza el rol de un usuario
    async fn update_role(&self, id: Uuid, rol: &str) -> Result<User, sqlx::Error>;
    
    /// Actualiza el estado de un usuario
    async fn update_status(&self, id: Uuid, activo: bool) -> Result<User, sqlx::Error>;
    
    /// Elimina lógicamente un usuario
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}
