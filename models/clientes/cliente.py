from pydantic import Field
from typing import Optional
from uuid import UUID

from ..base import TimestampedEntity, ContactInfoMixin
from ..enums import TipoClienteEnum

class Cliente(TimestampedEntity, ContactInfoMixin):
    """Entidad Cliente del dominio"""
    id_usuario: UUID = Field(..., description="Reference to User ID")
    tipo: TipoClienteEnum = Field(..., description="Client type: person or company")
    documento_identidad: str = Field(..., description="Client identification document")
    
    def is_empresa(self) -> bool:
        """Verifica si el cliente es una empresa"""
        return self.tipo == TipoClienteEnum.empresa
    
    def is_persona(self) -> bool:
        """Verifica si el cliente es una persona"""
        return self.tipo == TipoClienteEnum.persona