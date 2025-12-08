use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Tipos de dirección
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tipo_direccion", rename_all = "snake_case")]
pub enum TipoDireccion {
    Cliente,
    Almacen,
}

impl Default for TipoDireccion {
    fn default() -> Self {
        TipoDireccion::Cliente
    }
}

/// Entidad Direccion - Representa una dirección de cliente o almacén
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Direccion {
    /// ID único de la dirección
    pub id_direccion: Uuid,
    /// ID del perfil al que pertenece (puede ser null para almacenes)
    pub id_perfil: Option<Uuid>,
    /// Tipo de dirección: cliente o almacen
    pub tipo: String,
    /// Calle y número
    pub calle: String,
    /// Ciudad
    pub ciudad: String,
    /// Referencias adicionales (ej: "Casa azul", "Frente al parque")
    pub referencias_adicionales: Option<String>,
    /// País (default: Ecuador)
    pub pais: String,
    /// Latitud geográfica
    pub latitud: Decimal,
    /// Longitud geográfica
    pub longitud: Decimal,
    /// Si es la dirección predeterminada del perfil
    pub es_predeterminada: bool,
    /// Estado activo/inactivo (soft delete)
    pub activo: bool,
    /// Fecha de creación
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Fecha de última actualización
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
