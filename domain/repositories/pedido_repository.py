from abc import abstractmethod
from typing import Optional, List
from uuid import UUID
from .base_repository import IRepository

# Importamos solo los tipos, sin las implementaciones concretas
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from ...models.pedidos import Pedido

class IPedidoRepository(IRepository):
    """Interfaz para el repositorio de pedidos"""
    
    @abstractmethod
    def obtener_por_numero_tracking(self, numero_tracking: str) -> Optional['Pedido']:
        """Obtiene un pedido por su número de tracking"""
        pass
    
    @abstractmethod
    def obtener_por_cliente(self, cliente_id: UUID) -> List['Pedido']:
        """Obtiene todos los pedidos de un cliente"""
        pass
    
    @abstractmethod
    def obtener_por_transportista(self, transportista_id: UUID) -> List['Pedido']:
        """Obtiene todos los pedidos asignados a un transportista"""
        pass
    
    @abstractmethod
    def obtener_pendientes(self) -> List['Pedido']:
        """Obtiene todos los pedidos pendientes"""
        pass
    
    @abstractmethod
    def obtener_en_ruta(self) -> List['Pedido']:
        """Obtiene todos los pedidos en ruta"""
        pass