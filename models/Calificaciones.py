from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from .StandartModel import StandartModel

class Calificacion(StandartModel):
    # id heredado de StandartModel (UUID)
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    puntuacion: int = Field(..., description="Rating score from 1 to 5", ge=1, le=5)
    comentario: Optional[str] = Field(None, description="Rating comment")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_pedido": "550e8400-e29b-41d4-a716-446655440000",
                "puntuacion": 5,
                "comentario": "Excelente servicio, muy rápido y cuidadoso"
            }
        }