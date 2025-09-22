from pydantic import Field
from uuid import UUID
from datetime import date

from ..base import TimestampedEntity
from ..enums import EstadoRutaEnum

class Ruta(TimestampedEntity):
    """Entidad Ruta del dominio"""
    id_zona: UUID = Field(..., description="Reference to Zone ID")
    id_transportista: UUID = Field(..., description="Reference to Transporter ID")
    fecha: date = Field(..., description="Route date")
    estado: EstadoRutaEnum = Field(default=EstadoRutaEnum.pendiente, description="Route status")
    
    def iniciar_ruta(self):
        """Inicia la ruta"""
        self.estado = EstadoRutaEnum.en_progreso
        self.mark_updated()
    
    def completar_ruta(self):
        """Completa la ruta"""
        self.estado = EstadoRutaEnum.completada
        self.mark_updated()
    
    def is_completada(self) -> bool:
        """Verifica si la ruta está completada"""
        return self.estado == EstadoRutaEnum.completada

class DetalleRuta(TimestampedEntity):
    """Entidad DetalleRuta del dominio"""
    id_ruta: UUID = Field(..., description="Reference to Route ID")
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    orden_visita: int = Field(..., description="Visit order in the route", ge=1)
    estado: EstadoRutaEnum = Field(default=EstadoRutaEnum.pendiente, description="Detail route status")
    
    def marcar_completado(self):
        """Marca el detalle como completado"""
        self.estado = EstadoRutaEnum.completada
        self.mark_updated()