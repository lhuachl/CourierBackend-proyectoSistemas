"""
Esquemas Pydantic para pedidos
"""
from pydantic import BaseModel, Field
from typing import Optional
from datetime import datetime
from decimal import Decimal

class PedidoCreate(BaseModel):
    """Esquema para creación de pedido"""
    numero_tracking: str = Field(..., description="Número de tracking del pedido")
    id_cliente: str = Field(..., description="ID del cliente")
    fecha_entrega_estimada: datetime = Field(..., description="Fecha estimada de entrega")
    direccion_origen: str = Field(..., description="ID de dirección de origen")
    direccion_destino: str = Field(..., description="ID de dirección de destino")
    prioridad: str = Field(default="normal", description="Prioridad del pedido")
    peso: Decimal = Field(..., gt=0, description="Peso del pedido en kg")
    monto_total: Decimal = Field(..., gt=0, description="Monto total del pedido")

class PedidoUpdate(BaseModel):
    """Esquema para actualización de pedido"""
    estado: Optional[str] = Field(None, description="Estado del pedido")
    id_transportista: Optional[str] = Field(None, description="ID del transportista asignado")
    fecha_entrega_estimada: Optional[datetime] = Field(None, description="Nueva fecha estimada")

class PedidoResponse(BaseModel):
    """Esquema para respuesta de pedido"""
    id: str = Field(..., description="ID del pedido")
    numero_tracking: str = Field(..., description="Número de tracking")
    id_cliente: str = Field(..., description="ID del cliente")
    estado: str = Field(..., description="Estado del pedido")
    prioridad: str = Field(..., description="Prioridad del pedido")
    peso: Decimal = Field(..., description="Peso del pedido")
    monto_total: Decimal = Field(..., description="Monto total")
    fecha_solicitud: datetime = Field(..., description="Fecha de solicitud")
    fecha_entrega_estimada: datetime = Field(..., description="Fecha estimada de entrega")
    id_transportista: Optional[str] = Field(None, description="ID del transportista")
    
    class Config:
        from_attributes = True
