from abc import abstractmethod
from typing import Optional, List
from uuid import UUID
from .base_repository import IRepository

from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from ...models.transportes import Transportista

class ITransportistaRepository(IRepository):
    """Interfaz para el repositorio de transportistas"""
    
    @abstractmethod
    def obtener_disponibles(self) -> List['Transportista']:
        """Obtiene transportistas disponibles"""
        pass
    
    @abstractmethod
    def obtener_por_zona(self, zona_id: UUID) -> List['Transportista']:
        """Obtiene transportistas por zona"""
        pass
    
    @abstractmethod
    def obtener_por_tipo_vehiculo(self, tipo_vehiculo: str) -> List['Transportista']:
        """Obtiene transportistas por tipo de vehículo"""
        pass