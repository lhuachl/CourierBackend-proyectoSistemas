from pydantic import Field
from typing import Optional
from uuid import UUID
from datetime import datetime

from ..base import TimestampedEntity

class Calificacion(TimestampedEntity):
    """Entidad Calificación del dominio"""
    id_pedido: UUID = Field(..., description="Reference to Order ID")
    puntuacion: int = Field(..., description="Rating score from 1 to 5", ge=1, le=5)
    comentario: Optional[str] = Field(None, description="Rating comment")
    
    def is_excellent(self) -> bool:
        """Verifica si la calificación es excelente (5)"""
        return self.puntuacion == 5
    
    def is_poor(self) -> bool:
        """Verifica si la calificación es pobre (1-2)"""
        return self.puntuacion <= 2

class Notificacion(TimestampedEntity):
    """Entidad Notificación del dominio"""
    id_usuario: UUID = Field(..., description="Reference to User ID")
    id_pedido: Optional[UUID] = Field(None, description="Reference to Order ID")
    titulo: str = Field(..., description="Notification title")
    contenido: str = Field(..., description="Notification content")
    leido: bool = Field(default=False, description="Read status")
    fecha_envio: datetime = Field(..., description="Send date")
    
    def marcar_leido(self):
        """Marca la notificación como leída"""
        self.leido = True
        self.mark_updated()
    
    def is_leido(self) -> bool:
        """Verifica si la notificación fue leída"""
        return self.leido