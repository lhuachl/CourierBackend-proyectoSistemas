from pydantic import Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from decimal import Decimal

from ..base import TimestampedEntity, GeoLocationMixin
from ..enums import EstadoTrackingEnum

class Tracking(TimestampedEntity, GeoLocationMixin):
    """Entidad Tracking del dominio"""
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    id_transportista: Optional[UUID] = Field(None, description="Reference to Transporter ID")
    fecha_hora: datetime = Field(..., description="Tracking timestamp")
    estado: EstadoTrackingEnum = Field(..., description="Tracking status")
    comentario: Optional[str] = Field(None, description="Additional comments")
    
    def is_entregado(self) -> bool:
        """Verifica si el tracking indica entrega"""
        return self.estado == EstadoTrackingEnum.entregado
    
    def is_fallido(self) -> bool:
        """Verifica si el tracking indica fallo"""
        return self.estado == EstadoTrackingEnum.fallido
    
    def agregar_comentario(self, comentario: str):
        """Agrega un comentario al tracking"""
        self.comentario = comentario
        self.mark_updated()