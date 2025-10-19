from pydantic import Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from decimal import Decimal

from ..base import TimestampedEntity
from ..enums import EstadoPedidoEnum, PrioridadEnum

class Pedido(TimestampedEntity):
    """Entidad Pedido del dominio"""
    numero_tracking: str = Field(..., description="Unique tracking number")
    id_cliente: UUID = Field(..., description="Reference to Client ID")
    fecha_solicitud: datetime = Field(..., description="Request date")
    fecha_entrega_estimada: datetime = Field(..., description="Estimated delivery date")
    fecha_entrega_real: Optional[datetime] = Field(None, description="Actual delivery date")
    direccion_origen: UUID = Field(..., description="Origin address ID")
    direccion_destino: UUID = Field(..., description="Destination address ID")
    estado: EstadoPedidoEnum = Field(default=EstadoPedidoEnum.pendiente, description="Order status")
    prioridad: PrioridadEnum = Field(default=PrioridadEnum.normal, description="Order priority")
    peso: Decimal = Field(..., description="Package weight in kg", ge=0)
    dimensiones: Optional[str] = Field(None, description="Package dimensions")
    monto_total: Decimal = Field(..., description="Total amount", ge=0)
    id_transportista: Optional[UUID] = Field(None, description="Assigned transporter ID")
    
    def is_pendiente(self) -> bool:
        """Verifica si el pedido está pendiente"""
        return self.estado == EstadoPedidoEnum.pendiente
    
    def is_en_ruta(self) -> bool:
        """Verifica si el pedido está en ruta"""
        return self.estado == EstadoPedidoEnum.en_ruta
    
    def is_entregado(self) -> bool:
        """Verifica si el pedido fue entregado"""
        return self.estado == EstadoPedidoEnum.entregado
    
    def is_express(self) -> bool:
        """Verifica si el pedido es express"""
        return self.prioridad == PrioridadEnum.express
    
    def asignar_transportista(self, transportista_id: UUID):
        """Asigna un transportista al pedido"""
        self.id_transportista = transportista_id
        if self.estado == EstadoPedidoEnum.pendiente:
            self.estado = EstadoPedidoEnum.procesando
        self.mark_updated()
    
    def marcar_entregado(self):
        """Marca el pedido como entregado"""
        self.estado = EstadoPedidoEnum.entregado
        self.fecha_entrega_real = datetime.utcnow()
        self.mark_updated()
    
    def cancelar(self):
        """Cancela el pedido"""
        if self.estado not in [EstadoPedidoEnum.entregado]:
            self.estado = EstadoPedidoEnum.cancelado
            self.mark_updated()
    
    @property
    def dias_desde_solicitud(self) -> int:
        """Retorna los días transcurridos desde la solicitud"""
        return (datetime.utcnow() - self.fecha_solicitud).days