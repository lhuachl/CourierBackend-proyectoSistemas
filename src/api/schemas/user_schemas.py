"""
Esquemas Pydantic para usuarios
"""
from pydantic import BaseModel, EmailStr, Field
from typing import Optional
from datetime import datetime

class UserResponse(BaseModel):
    """Esquema para respuesta de usuario"""
    id: str = Field(..., description="ID del usuario")
    email: EmailStr = Field(..., description="Email del usuario")
    nombre: str = Field(..., description="Nombre del usuario")
    apellido: str = Field(..., description="Apellido del usuario")
    role: str = Field(..., description="Rol del usuario")
    estado: str = Field(..., description="Estado del usuario")
    created_at: datetime = Field(..., description="Fecha de creación")
    
    class Config:
        from_attributes = True

class UserUpdate(BaseModel):
    """Esquema para actualización de usuario"""
    nombre: Optional[str] = Field(None, min_length=2, description="Nombre del usuario")
    apellido: Optional[str] = Field(None, min_length=2, description="Apellido del usuario")
    estado: Optional[str] = Field(None, description="Estado del usuario")
