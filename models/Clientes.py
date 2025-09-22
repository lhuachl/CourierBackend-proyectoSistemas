from pydantic import BaseModel, Field
from typing import Optional
from uuid import UUID
from .StandartModel import StandartModel
from .TipoClienteEnum import TipoClienteEnum

class Cliente(StandartModel):
    # id heredado de StandartModel (UUID)
    id_usuario: UUID = Field(..., description="Reference to User ID")
    tipo: TipoClienteEnum = Field(..., description="Client type: person or company")
    documento_identidad: str = Field(..., description="Client identification document", unique=True)
    telefono: Optional[str] = Field(None, description="Client phone number")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "id_usuario": "550e8400-e29b-41d4-a716-446655440000",
                "tipo": "persona",
                "documento_identidad": "12345678",
                "telefono": "+51987654321"
            }
        }
