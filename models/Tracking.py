from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from decimal import Decimal
from .StandartModel import StandartModel
from .EstadoTrackingEnum import EstadoTrackingEnum

class Tracking(StandartModel):
    # id heredado de StandartModel (UUID)
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    id_transportista: Optional[UUID] = Field(None, description="Reference to Transporter ID")
    fecha_hora: datetime = Field(..., description="Tracking timestamp")
    latitud: Decimal = Field(..., description="Latitude coordinate", ge=-90, le=90)
    longitud: Decimal = Field(..., description="Longitude coordinate", ge=-180, le=180)
    estado: EstadoTrackingEnum = Field(..., description="Tracking status")
    comentario: Optional[str] = Field(None, description="Additional comments")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_pedido": "550e8400-e29b-41d4-a716-446655440000",
                "id_transportista": "550e8400-e29b-41d4-a716-446655440001",
                "fecha_hora": "2023-01-01T14:30:00Z",
                "latitud": -12.0464,
                "longitud": -77.0428,
                "estado": "en_ruta",
                "comentario": "Paquete recogido y en camino"
            }
        }