from pydantic import BaseModel, Field
from uuid import UUID
from .StandartModel import StandartModel
from .EstadoRutaEnum import EstadoRutaEnum

class DetalleRuta(StandartModel):
    # id heredado de StandartModel (UUID)
    id_ruta: UUID = Field(..., description="Reference to Route ID")
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    orden_visita: int = Field(..., description="Visit order in the route", ge=1)
    estado: EstadoRutaEnum = Field(default=EstadoRutaEnum.pendiente, description="Detail route status")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_ruta": "550e8400-e29b-41d4-a716-446655440000",
                "id_pedido": "550e8400-e29b-41d4-a716-446655440001",
                "orden_visita": 1,
                "estado": "pendiente"
            }
        }