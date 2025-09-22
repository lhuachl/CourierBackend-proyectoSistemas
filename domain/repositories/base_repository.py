from abc import ABC, abstractmethod
from typing import Optional, List
from uuid import UUID

# Repositorio base
class IRepository(ABC):
    """Interfaz base para todos los repositorios"""
    
    @abstractmethod
    def guardar(self, entidad) -> None:
        """Guarda una entidad"""
        pass
    
    @abstractmethod
    def obtener_por_id(self, id: UUID) -> Optional[object]:
        """Obtiene una entidad por su ID"""
        pass
    
    @abstractmethod
    def eliminar(self, id: UUID) -> None:
        """Elimina una entidad por su ID"""
        pass