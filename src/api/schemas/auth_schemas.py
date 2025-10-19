"""
Esquemas Pydantic para autenticación
"""
from pydantic import BaseModel, EmailStr, Field

class LoginRequest(BaseModel):
    """Esquema para solicitud de login"""
    email: EmailStr = Field(..., description="Email del usuario")
    password: str = Field(..., min_length=6, description="Contraseña del usuario")

class RegisterRequest(BaseModel):
    """Esquema para registro de usuario"""
    email: EmailStr = Field(..., description="Email del usuario")
    password: str = Field(..., min_length=6, description="Contraseña del usuario")
    nombre: str = Field(..., min_length=2, description="Nombre del usuario")
    apellido: str = Field(..., min_length=2, description="Apellido del usuario")
    role: str = Field(default="cliente", description="Rol del usuario")

class TokenResponse(BaseModel):
    """Esquema para respuesta de token"""
    access_token: str = Field(..., description="Token de acceso JWT")
    token_type: str = Field(default="bearer", description="Tipo de token")
    user_id: str = Field(..., description="ID del usuario")
    email: str = Field(..., description="Email del usuario")
    role: str = Field(..., description="Rol del usuario")
