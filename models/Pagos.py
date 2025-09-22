from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from decimal import Decimal
from .StandartModel import StandartModel
from .MetodoPagoEnum import MetodoPagoEnum
from .EstadoPagoEnum import EstadoPagoEnum

class Pago(StandartModel):
    # id heredado de StandartModel (UUID)
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    monto: Decimal = Field(..., description="Payment amount", ge=0)
    metodo_pago: MetodoPagoEnum = Field(..., description="Payment method")
    estado: EstadoPagoEnum = Field(default=EstadoPagoEnum.pendiente, description="Payment status")
    referencia_externa: Optional[str] = Field(None, description="External payment reference")
    fecha_pago: Optional[datetime] = Field(None, description="Payment date")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_pedido": "550e8400-e29b-41d4-a716-446655440000",
                "monto": 25.00,
                "metodo_pago": "tarjeta",
                "estado": "completado",
                "referencia_externa": "TXN123456789",
                "fecha_pago": "2023-01-01T15:30:00Z"
            }
        }