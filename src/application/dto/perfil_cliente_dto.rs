use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::PerfilCliente;

// ============================================================================
// REQUEST DTOs
// ============================================================================

/// DTO para crear un perfil de cliente
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePerfilClienteDTO {
    /// Documento de identidad (cédula, RUC, pasaporte)
    #[schema(example = "1234567890")]
    pub documento_identidad: Option<String>,
    
    /// Número de teléfono
    #[schema(example = "+593999123456")]
    pub telefono: Option<String>,
}

/// DTO para actualizar un perfil de cliente
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePerfilClienteDTO {
    /// Documento de identidad (cédula, RUC, pasaporte)
    #[schema(example = "1234567890")]
    pub documento_identidad: Option<String>,
    
    /// Número de teléfono
    #[schema(example = "+593999123456")]
    pub telefono: Option<String>,
}

// ============================================================================
// RESPONSE DTOs
// ============================================================================

/// DTO de respuesta para un perfil de cliente
#[derive(Debug, Serialize, ToSchema)]
pub struct PerfilClienteResponseDTO {
    /// ID único del perfil
    pub id_perfil: Uuid,
    
    /// ID del usuario asociado
    pub id_usuario: Uuid,
    
    /// Documento de identidad
    pub documento_identidad: Option<String>,
    
    /// Número de teléfono
    pub telefono: Option<String>,
    
    /// Fecha de creación
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Fecha de última actualización
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de respuesta para lista de perfiles
#[derive(Debug, Serialize, ToSchema)]
pub struct PerfilesClienteListResponseDTO {
    /// Lista de perfiles
    pub perfiles: Vec<PerfilClienteResponseDTO>,
    
    /// Total de perfiles
    pub total: usize,
}

// ============================================================================
// CONVERSIONES (From impls)
// ============================================================================

impl From<PerfilCliente> for PerfilClienteResponseDTO {
    fn from(perfil: PerfilCliente) -> Self {
        Self {
            id_perfil: perfil.id_perfil,
            id_usuario: perfil.id_usuario,
            documento_identidad: perfil.documento_identidad,
            telefono: perfil.telefono,
            created_at: perfil.created_at,
            updated_at: perfil.updated_at,
        }
    }
}

impl From<Vec<PerfilCliente>> for PerfilesClienteListResponseDTO {
    fn from(perfiles: Vec<PerfilCliente>) -> Self {
        let total = perfiles.len();
        Self {
            perfiles: perfiles.into_iter().map(Into::into).collect(),
            total,
        }
    }
}
