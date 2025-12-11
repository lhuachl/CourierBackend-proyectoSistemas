use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

// ============================================================================
// CREATE USER DTO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDTO {
    /// Email del usuario (sincronizado desde Supabase)
    pub email: String,
    
    /// Nombre del usuario
    pub nombre: Option<String>,
    
    /// Apellido del usuario
    pub apellido: Option<String>,
    
    /// Rol en el sistema (cliente, transportista, admin)
    #[serde(default = "default_role")]
    pub rol: String,
}

fn default_role() -> String {
    "cliente".to_string()
}

// ============================================================================
// UPDATE USER DTO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserDTO {
    /// Email (sincronizado desde Supabase)
    pub email: Option<String>,
    
    /// Nombre del usuario
    pub nombre: Option<String>,
    
    /// Apellido del usuario
    pub apellido: Option<String>,
    
    /// Rol en el sistema (solo admin puede cambiar)
    pub rol: Option<String>,
    
    /// URL de foto de perfil
    pub foto_perfil: Option<String>,
}

// ============================================================================
// UPDATE USER ROLE DTO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRoleDTO {
    /// Nuevo rol (cliente, transportista, admin)
    pub rol: String,
}

// ============================================================================
// UPDATE USER STATUS DTO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserStatusDTO {
    /// Estado del usuario (activo o suspendido)
    pub activo: bool,
}

// ============================================================================
// USER RESPONSE DTO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDTO {
    /// ID del usuario
    pub id: Uuid,
    
    /// Email del usuario
    pub email: Option<String>,
    
    /// Nombre del usuario
    pub nombre: Option<String>,
    
    /// Apellido del usuario
    pub apellido: Option<String>,
    
    /// Rol en el sistema
    pub rol: String,
    
    /// URL de foto de perfil
    pub foto_perfil: Option<String>,
    
    /// Estado del usuario
    pub activo: bool,
    
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// USERS LIST RESPONSE DTO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UsersListResponseDTO {
    /// Total de usuarios
    pub total: i64,
    
    /// Lista de usuarios
    pub users: Vec<UserResponseDTO>,
}
