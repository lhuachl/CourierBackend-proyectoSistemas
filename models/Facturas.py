from pydantic import BaseModel, Field
from uuid import UUID
from datetime import datetime
from decimal import Decimal
from .StandartModel import StandartModel
from .EstadoFacturaEnum import EstadoFacturaEnum

class Factura(StandartModel):
    # id heredado de StandartModel (UUID)
    numero_factura: str = Field(..., description="Unique invoice number", unique=True)
    id_cliente: UUID = Field(..., description="Reference to Client ID")
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    fecha_emision: datetime = Field(..., description="Invoice emission date")
    subtotal: Decimal = Field(..., description="Subtotal amount", ge=0)
    impuestos: Decimal = Field(..., description="Tax amount", ge=0)
    total: Decimal = Field(..., description="Total amount", ge=0)
    estado: EstadoFacturaEnum = Field(default=EstadoFacturaEnum.emitida, description="Invoice status")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "numero_factura": "FAC-2023-001234",
                "id_cliente": "550e8400-e29b-41d4-a716-446655440000",
                "id_pedido": "550e8400-e29b-41d4-a716-446655440001",
                "fecha_emision": "2023-01-01T16:00:00Z",
                "subtotal": 21.19,
                "impuestos": 3.81,
                "total": 25.00,
                "estado": "emitida"
            }
        }