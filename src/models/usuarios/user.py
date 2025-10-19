from pydantic import Field, EmailStr
from typing import Optional
from uuid import UUID

from ..base import TimestampedEntity
from ..enums import RoleEnum, EstadoEnum

class User(TimestampedEntity):
    """Entidad User del dominio - Representa un usuario del sistema"""
    email: EmailStr = Field(..., description="User email address")
    password_hash: str = Field(..., description="Hashed password")
    nombre: str = Field(..., description="User first name")
    apellido: str = Field(..., description="User last name")
    role: RoleEnum = Field(..., description="User role")
    estado: EstadoEnum = Field(default=EstadoEnum.activo, description="User status")
    
    def is_active(self) -> bool:
        """Verifica si el usuario está activo"""
        return self.estado == EstadoEnum.activo
    
    def is_admin(self) -> bool:
        """Verifica si el usuario es administrador"""
        return self.role == RoleEnum.admin
    
    def can_create_orders(self) -> bool:
        """Verifica si el usuario puede crear pedidos"""
        return self.role in [RoleEnum.cliente, RoleEnum.admin]
    
    def can_deliver_orders(self) -> bool:
        """Verifica si el usuario puede entregar pedidos"""
        return self.role in [RoleEnum.transportista, RoleEnum.admin]
    
    @property
    def full_name(self) -> str:
        """Retorna el nombre completo del usuario"""
        return f"{self.nombre} {self.apellido}"