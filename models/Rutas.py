from pydantic import BaseModel, Field
from uuid import UUID
from datetime import date
from .StandartModel import StandartModel
from .EstadoRutaEnum import EstadoRutaEnum

class Ruta(StandartModel):
    # id heredado de StandartModel (UUID)
    id_zona: UUID = Field(..., description="Reference to Zone ID")
    id_transportista: UUID = Field(..., description="Reference to Transporter ID")
    fecha: date = Field(..., description="Route date")
    estado: EstadoRutaEnum = Field(default=EstadoRutaEnum.pendiente, description="Route status")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_zona": "550e8400-e29b-41d4-a716-446655440000",
                "id_transportista": "550e8400-e29b-41d4-a716-446655440001",
                "fecha": "2023-01-01",
                "estado": "pendiente"
            }
        }