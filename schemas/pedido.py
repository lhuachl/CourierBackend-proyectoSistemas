from datetime import datetime
from decimal import Decimal
from typing import Optional
from uuid import UUID

from pydantic import BaseModel, Field
from models.enums import EstadoPedidoEnum, PrioridadEnum


class PedidoCreate(BaseModel):
    numero_tracking: str = Field(..., description="Unique tracking number")
    id_cliente: UUID = Field(..., description="Reference to Client ID")
    fecha_solicitud: datetime = Field(..., description="Request date")
    fecha_entrega_estimada: datetime = Field(..., description="Estimated delivery date")
    direccion_origen: UUID = Field(..., description="Origin address ID")
    direccion_destino: UUID = Field(..., description="Destination address ID")
    prioridad: PrioridadEnum = Field(default=PrioridadEnum.normal)
    peso: Decimal = Field(..., ge=0)
    dimensiones: Optional[str] = None
    monto_total: Decimal = Field(..., ge=0)


class PedidoUpdate(BaseModel):
    estado: Optional[EstadoPedidoEnum] = None
    fecha_entrega_real: Optional[datetime] = None
    id_transportista: Optional[UUID] = None
    prioridad: Optional[PrioridadEnum] = None


class PedidoResponse(BaseModel):
    id: UUID
    numero_tracking: str
    id_cliente: UUID
    fecha_solicitud: datetime
    fecha_entrega_estimada: datetime
    fecha_entrega_real: Optional[datetime]
    direccion_origen: UUID
    direccion_destino: UUID
    estado: EstadoPedidoEnum
    prioridad: PrioridadEnum
    peso: Decimal
    dimensiones: Optional[str]
    monto_total: Decimal
    id_transportista: Optional[UUID]

    model_config = {
        "from_attributes": True
    }
