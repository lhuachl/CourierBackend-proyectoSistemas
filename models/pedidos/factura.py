from pydantic import Field
from uuid import UUID
from datetime import datetime
from decimal import Decimal

from ..base import TimestampedEntity
from ..enums import EstadoFacturaEnum

class Factura(TimestampedEntity):
    """Entidad Factura del dominio"""
    numero_factura: str = Field(..., description="Unique invoice number")
    id_cliente: UUID = Field(..., description="Reference to Client ID")
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    fecha_emision: datetime = Field(..., description="Invoice emission date")
    subtotal: Decimal = Field(..., description="Subtotal amount", ge=0)
    impuestos: Decimal = Field(..., description="Tax amount", ge=0)
    total: Decimal = Field(..., description="Total amount", ge=0)
    estado: EstadoFacturaEnum = Field(default=EstadoFacturaEnum.emitida, description="Invoice status")
    
    def marcar_pagada(self):
        """Marca la factura como pagada"""
        self.estado = EstadoFacturaEnum.pagada
        self.mark_updated()
    
    def anular(self):
        """Anula la factura"""
        self.estado = EstadoFacturaEnum.anulada
        self.mark_updated()
    
    def is_pagada(self) -> bool:
        """Verifica si la factura está pagada"""
        return self.estado == EstadoFacturaEnum.pagada
    
    def calcular_total(self):
        """Calcula el total basado en subtotal e impuestos"""
        self.total = self.subtotal + self.impuestos
        self.mark_updated()