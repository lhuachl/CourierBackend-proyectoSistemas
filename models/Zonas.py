from pydantic import BaseModel, Field
from typing import Dict, Any
from .StandartModel import StandartModel

class Zona(StandartModel):
    # id heredado de StandartModel (UUID)
    nombre: str = Field(..., description="Zone name")
    ciudad: str = Field(..., description="City name")
    poligono_geo: Dict[str, Any] = Field(..., description="GeoJSON polygon representation")
    
    class Config:
        orm_mode = True
        schema_extra = {
            "example": {
                "nombre": "San Isidro",
                "ciudad": "Lima",
                "poligono_geo": {
                    "type": "Polygon",
                    "coordinates": [[
                        [-77.0428, -12.0464],
                        [-77.0328, -12.0464],
                        [-77.0328, -12.0364],
                        [-77.0428, -12.0364],
                        [-77.0428, -12.0464]
                    ]]
                }
            }
        }