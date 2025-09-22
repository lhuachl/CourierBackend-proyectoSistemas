from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from decimal import Decimal
from .StandartModel import StandartModel
from .TipoDireccionEnum import TipoDireccionEnum

class Direccion(StandartModel):
    # id heredado de StandartModel (UUID)
    id_cliente: Optional[UUID] = Field(None, description="Reference to Client ID")
    calle: str = Field(..., description="Street name")
    numero: str = Field(..., description="Street number")
    ciudad: str = Field(..., description="City name")
    codigo_postal: str = Field(..., description="Postal code")
    latitud: Decimal = Field(..., description="Latitude coordinate", ge=-90, le=90)
    longitud: Decimal = Field(..., description="Longitude coordinate", ge=-180, le=180)
    tipo: TipoDireccionEnum = Field(..., description="Address type")
    id_zona: Optional[UUID] = Field(None, description="Reference to Zone ID")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_cliente": "550e8400-e29b-41d4-a716-446655440000",
                "calle": "Av. Javier Prado Este",
                "numero": "1234",
                "ciudad": "Lima",
                "codigo_postal": "15036",
                "latitud": -12.0464,
                "longitud": -77.0428,
                "tipo": "residencial",
                "id_zona": "550e8400-e29b-41d4-a716-446655440001"
            }
        }