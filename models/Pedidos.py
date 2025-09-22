from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from decimal import Decimal
from .StandartModel import StandartModel
from .EstadoPedidoEnum import EstadoPedidoEnum
from .PrioridadEnum import PrioridadEnum

class Pedido(StandartModel):
    # id heredado de StandartModel (UUID)
    numero_tracking: str = Field(..., description="Unique tracking number", unique=True)
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
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "numero_tracking": "TRK123456789",
                "id_cliente": "550e8400-e29b-41d4-a716-446655440000",
                "fecha_solicitud": "2023-01-01T10:00:00Z",
                "fecha_entrega_estimada": "2023-01-02T15:00:00Z",
                "direccion_origen": "550e8400-e29b-41d4-a716-446655440001",
                "direccion_destino": "550e8400-e29b-41d4-a716-446655440002",
                "estado": "pendiente",
                "prioridad": "normal",
                "peso": 2.5,
                "dimensiones": "30x20x10 cm",
                "monto_total": 25.00
            }
        }