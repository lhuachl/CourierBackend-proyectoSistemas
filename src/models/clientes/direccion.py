from pydantic import Field
from typing import Optional
from uuid import UUID

from ..base import TimestampedEntity, GeoLocationMixin, AddressMixin
from ..enums import TipoDireccionEnum

class Direccion(TimestampedEntity, GeoLocationMixin, AddressMixin):
    """Entidad Dirección del dominio"""
    id_cliente: Optional[UUID] = Field(None, description="Reference to Client ID")
    tipo: TipoDireccionEnum = Field(..., description="Address type")
    id_zona: Optional[UUID] = Field(None, description="Reference to Zone ID")
    
    def is_comercial(self) -> bool:
        """Verifica si la dirección es comercial"""
        return self.tipo == TipoDireccionEnum.comercial
    
    def is_residencial(self) -> bool:
        """Verifica si la dirección es residencial"""
        return self.tipo == TipoDireccionEnum.residencial
    
    @property
    def direccion_completa(self) -> str:
        """Retorna la dirección completa como string"""
        return f"{self.calle} {self.numero}, {self.ciudad} {self.codigo_postal}"