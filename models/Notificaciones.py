from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from datetime import datetime
from .StandartModel import StandartModel

class Notificacion(StandartModel):
    # id heredado de StandartModel (UUID)
    id_usuario: UUID = Field(..., description="Reference to User ID")
    id_pedido: Optional[UUID] = Field(None, description="Reference to Order ID")
    titulo: str = Field(..., description="Notification title")
    contenido: str = Field(..., description="Notification content")
    leido: bool = Field(default=False, description="Read status")
    fecha_envio: datetime = Field(..., description="Send date")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_usuario": "550e8400-e29b-41d4-a716-446655440000",
                "id_pedido": "550e8400-e29b-41d4-a716-446655440001",
                "titulo": "Pedido en camino",
                "contenido": "Tu pedido TRK123456789 está en camino y llegará en 30 minutos",
                "leido": False,
                "fecha_envio": "2023-01-01T14:30:00Z"
            }
        }