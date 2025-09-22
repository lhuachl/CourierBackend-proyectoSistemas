from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from decimal import Decimal
from .StandartModel import StandartModel
from .TipoVehiculoEnum import TipoVehiculoEnum
from .EstadoTransportistaEnum import EstadoTransportistaEnum

class Transportista(StandartModel):
    # id heredado de StandartModel (UUID)
    id_usuario: UUID = Field(..., description="Reference to User ID")
    tipo_vehiculo: TipoVehiculoEnum = Field(..., description="Vehicle type")
    placa_vehiculo: str = Field(..., description="Vehicle license plate")
    capacidad_carga: Decimal = Field(..., description="Load capacity in kg", ge=0)
    estado: EstadoTransportistaEnum = Field(default=EstadoTransportistaEnum.disponible, description="Transporter status")
    zona_asignada: Optional[UUID] = Field(None, description="Assigned zone ID")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_usuario": "550e8400-e29b-41d4-a716-446655440000",
                "tipo_vehiculo": "moto",
                "placa_vehiculo": "ABC-123",
                "capacidad_carga": 50.0,
                "estado": "disponible",
                "zona_asignada": "550e8400-e29b-41d4-a716-446655440001"
            }
        }