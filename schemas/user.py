from typing import Optional
from uuid import UUID

from pydantic import BaseModel, EmailStr, Field
from models.enums import RoleEnum, EstadoEnum


class UserCreate(BaseModel):
    email: EmailStr
    password: str = Field(..., min_length=6)
    nombre: str
    apellido: str
    role: RoleEnum


class UserLogin(BaseModel):
    email: EmailStr
    password: str


class UserResponse(BaseModel):
    id: UUID
    email: EmailStr
    nombre: str
    apellido: str
    role: RoleEnum
    estado: EstadoEnum

    model_config = {
        "from_attributes": True
    }
