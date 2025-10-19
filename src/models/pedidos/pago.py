from pydantic import Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from decimal import Decimal

from ..base import TimestampedEntity
from ..enums import MetodoPagoEnum, EstadoPagoEnum

class Pago(TimestampedEntity):
    """Entidad Pago del dominio"""
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    monto: Decimal = Field(..., description="Payment amount", ge=0)
    metodo_pago: MetodoPagoEnum = Field(..., description="Payment method")
    estado: EstadoPagoEnum = Field(default=EstadoPagoEnum.pendiente, description="Payment status")
    referencia_externa: Optional[str] = Field(None, description="External payment reference")
    fecha_pago: Optional[datetime] = Field(None, description="Payment date")
    
    def marcar_completado(self, referencia: Optional[str] = None):
        """Marca el pago como completado"""
        self.estado = EstadoPagoEnum.completado
        self.fecha_pago = datetime.utcnow()
        if referencia:
            self.referencia_externa = referencia
        self.mark_updated()
    
    def marcar_fallido(self):
        """Marca el pago como fallido"""
        self.estado = EstadoPagoEnum.fallido
        self.mark_updated()
    
    def is_completado(self) -> bool:
        """Verifica si el pago está completado"""
        return self.estado == EstadoPagoEnum.completado
    
    def is_efectivo(self) -> bool:
        """Verifica si el pago es en efectivo"""
        return self.metodo_pago == MetodoPagoEnum.efectivo