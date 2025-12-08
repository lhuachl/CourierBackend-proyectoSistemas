use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Claims del JWT de Supabase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Audience - siempre "authenticated" para usuarios autenticados
    pub aud: String,
    /// Tiempo de expiración (Unix timestamp)
    pub exp: i64,
    /// Tiempo de emisión (Unix timestamp)
    pub iat: i64,
    /// Issuer - URL de Supabase Auth
    pub iss: String,
    /// Subject - UUID del usuario
    pub sub: Uuid,
    /// Email del usuario
    pub email: Option<String>,
    /// Rol en Supabase (authenticated, anon, etc.)
    pub role: Option<String>,
}

impl Claims {
    /// Obtiene el ID del usuario
    pub fn user_id(&self) -> Uuid {
        self.sub
    }

    /// Verifica si el token está expirado
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        self.exp < now
    }
}

/// Usuario autenticado extraído del JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: Option<String>,
    pub role: Option<String>,
}

impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        Self {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
        }
    }
}
