from pydantic import BaseModel, Field, EmailStr
from typing import Optional
from uuid import UUID
from .StandartModel import StandartModel
from .RoleEnum import RoleEnum
from .EstadoEnum import EstadoEnum

class User(StandartModel):
    # id heredado de StandartModel (UUID)
    email: EmailStr = Field(..., description="User email address", unique=True)
    password_hash: str = Field(..., description="Hashed password")
    nombre: str = Field(..., description="User first name")
    apellido: str = Field(..., description="User last name")
    role: RoleEnum = Field(..., description="User role")
    estado: EstadoEnum = Field(default=EstadoEnum.activo, description="User status")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "email": "usuario@ejemplo.com",
                "nombre": "Juan",
                "apellido": "Pérez",
                "role": "cliente",
                "estado": "activo"
            }
        }

