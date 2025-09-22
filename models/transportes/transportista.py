from pydantic import Field
from typing import Optional
from uuid import UUID
from decimal import Decimal

from ..base import TimestampedEntity
from ..enums import TipoVehiculoEnum, EstadoTransportistaEnum

class Transportista(TimestampedEntity):
    """Entidad Transportista del dominio"""
    id_usuario: UUID = Field(..., description="Reference to User ID")
    tipo_vehiculo: TipoVehiculoEnum = Field(..., description="Vehicle type")
    placa_vehiculo: str = Field(..., description="Vehicle license plate")
    capacidad_carga: Decimal = Field(..., description="Load capacity in kg", ge=0)
    estado: EstadoTransportistaEnum = Field(default=EstadoTransportistaEnum.disponible, description="Transporter status")
    zona_asignada: Optional[UUID] = Field(None, description="Assigned zone ID")
    
    def is_disponible(self) -> bool:
        """Verifica si el transportista está disponible"""
        return self.estado == EstadoTransportistaEnum.disponible
    
    def is_en_ruta(self) -> bool:
        """Verifica si el transportista está en ruta"""
        return self.estado == EstadoTransportistaEnum.en_ruta
    
    def marcar_en_ruta(self):
        """Marca al transportista como en ruta"""
        self.estado = EstadoTransportistaEnum.en_ruta
        self.mark_updated()
    
    def marcar_disponible(self):
        """Marca al transportista como disponible"""
        self.estado = EstadoTransportistaEnum.disponible
        self.mark_updated()
    
    def asignar_zona(self, zona_id: UUID):
        """Asigna una zona al transportista"""
        self.zona_asignada = zona_id
        self.mark_updated()
    
    def puede_cargar(self, peso: Decimal) -> bool:
        """Verifica si puede cargar un peso específico"""
        return peso <= self.capacidad_carga