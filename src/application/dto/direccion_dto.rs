use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::Direccion;

// ============================================================================
// REQUEST DTOs
// ============================================================================

/// DTO para crear una dirección
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateDireccionDTO {
    /// Tipo de dirección: "cliente" o "almacen"
    #[schema(example = "cliente")]
    pub tipo: Option<String>,
    
    /// Calle y número
    #[schema(example = "Av. Amazonas N34-45 y Atahualpa")]
    pub calle: String,
    
    /// Ciudad
    #[schema(example = "Quito")]
    pub ciudad: String,
    
    /// País (default: Ecuador)
    #[schema(example = "Ecuador")]
    pub pais: Option<String>,
    
    /// Referencias adicionales
    #[schema(example = "Edificio Torre Azul, piso 5, oficina 502")]
    pub referencias_adicionales: Option<String>,
    
    /// Latitud geográfica
    #[schema(example = -0.180653)]
    pub latitud: f64,
    
    /// Longitud geográfica
    #[schema(example = -78.467834)]
    pub longitud: f64,
    
    /// Si es la dirección predeterminada
    #[schema(example = false)]
    pub es_predeterminada: Option<bool>,
}

/// DTO para actualizar una dirección
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateDireccionDTO {
    /// Calle y número
    pub calle: Option<String>,
    
    /// Ciudad
    pub ciudad: Option<String>,
    
    /// País
    pub pais: Option<String>,
    
    /// Referencias adicionales
    pub referencias_adicionales: Option<String>,
    
    /// Latitud geográfica
    pub latitud: Option<f64>,
    
    /// Longitud geográfica
    pub longitud: Option<f64>,
}

/// DTO para crear un almacén (admin)
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateAlmacenDTO {
    /// Nombre o identificador del almacén
    #[schema(example = "Bodega Central Quito")]
    pub nombre: String,
    
    /// Calle y número
    #[schema(example = "Panamericana Norte Km 5.5")]
    pub calle: String,
    
    /// Ciudad
    #[schema(example = "Quito")]
    pub ciudad: String,
    
    /// País (default: Ecuador)
    pub pais: Option<String>,
    
    /// Referencias adicionales
    pub referencias_adicionales: Option<String>,
    
    /// Latitud geográfica
    pub latitud: f64,
    
    /// Longitud geográfica
    pub longitud: f64,
}

// ============================================================================
// RESPONSE DTOs
// ============================================================================

/// DTO de respuesta para una dirección
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DireccionResponseDTO {
    /// ID único de la dirección
    pub id_direccion: Uuid,
    
    /// ID del perfil (null para almacenes)
    pub id_perfil: Option<Uuid>,
    
    /// Tipo: "cliente" o "almacen"
    pub tipo: String,
    
    /// Calle y número
    pub calle: String,
    
    /// Ciudad
    pub ciudad: String,
    
    /// País
    pub pais: String,
    
    /// Referencias adicionales
    pub referencias_adicionales: Option<String>,
    
    /// Latitud
    pub latitud: f64,
    
    /// Longitud
    pub longitud: f64,
    
    /// Si es predeterminada
    pub es_predeterminada: bool,
    
    /// Si está activa
    pub activo: bool,
    
    /// Fecha de creación
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Fecha de última actualización
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// DTO de respuesta para lista de direcciones
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DireccionesListResponseDTO {
    /// Lista de direcciones
    pub direcciones: Vec<DireccionResponseDTO>,
    
    /// Total de direcciones
    pub total: usize,
}

// ============================================================================
// CONVERSIONES (From impls)
// ============================================================================

impl From<Direccion> for DireccionResponseDTO {
    fn from(d: Direccion) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id_direccion: d.id_direccion,
            id_perfil: d.id_perfil,
            tipo: d.tipo,
            calle: d.calle,
            ciudad: d.ciudad,
            pais: d.pais,
            referencias_adicionales: d.referencias_adicionales,
            latitud: d.latitud.to_f64().unwrap_or(0.0),
            longitud: d.longitud.to_f64().unwrap_or(0.0),
            es_predeterminada: d.es_predeterminada,
            activo: d.activo,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

impl From<Vec<Direccion>> for DireccionesListResponseDTO {
    fn from(direcciones: Vec<Direccion>) -> Self {
        let total = direcciones.len();
        Self {
            direcciones: direcciones.into_iter().map(Into::into).collect(),
            total,
        }
    }
}
