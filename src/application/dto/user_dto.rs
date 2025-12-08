use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUserDTO {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub rol: String,
    pub gmail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserDTO {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub foto_perfil: Option<String>,
    pub gmail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDTO {
    pub id: Uuid,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub rol: String,
}
