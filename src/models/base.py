from pydantic import BaseModel, Field
from typing import Optional
from datetime import datetime
from uuid import UUID, uuid4

class BaseEntity(BaseModel):
    """Entidad base del dominio - Agnóstica a tecnología"""
    id: Optional[UUID] = Field(default_factory=uuid4, description="Unique identifier")
    
    class Config:
        # Sin orm_mode para mantener agnóstico a tecnología
        validate_assignment = True
        use_enum_values = True

class TimestampedEntity(BaseEntity):
    """Entidad con timestamps automáticos"""
    created_at: Optional[datetime] = Field(default_factory=datetime.utcnow, description="Creation timestamp")
    updated_at: Optional[datetime] = Field(default_factory=datetime.utcnow, description="Last update timestamp")
    
    def mark_updated(self):
        """Marca la entidad como actualizada"""
        self.updated_at = datetime.utcnow()

class StandartModel(TimestampedEntity):
    """Modelo estándar para compatibilidad con código existente"""
    pass

# Mixins reutilizables
class GeoLocationMixin(BaseModel):
    """Mixin para entidades con geolocalización"""
    latitud: float = Field(..., description="Latitude coordinate", ge=-90, le=90)
    longitud: float = Field(..., description="Longitude coordinate", ge=-180, le=180)

class ContactInfoMixin(BaseModel):
    """Mixin para información de contacto"""
    telefono: Optional[str] = Field(None, description="Phone number")

class AddressMixin(BaseModel):
    """Mixin para direcciones"""
    calle: str = Field(..., description="Street name")
    numero: str = Field(..., description="Street number")
    ciudad: str = Field(..., description="City name")
    codigo_postal: str = Field(..., description="Postal code")