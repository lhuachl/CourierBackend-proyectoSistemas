"""
Ejemplo de uso de los repositorios en un caso de uso/servicio de aplicación
"""

from typing import Optional
from uuid import UUID

# Interfaces del dominio - NO implementaciones concretas
from ...domain.repositories.pedido_repository import IPedidoRepository
from ...domain.repositories.transportista_repository import ITransportistaRepository

# Entidades del modelo
from ...models.pedidos import Pedido
from ...models.enums import EstadoPedidoEnum

class PedidoService:
    """Servicio de aplicación para manejar pedidos"""
    
    def __init__(
        self, 
        pedido_repo: IPedidoRepository,
        transportista_repo: ITransportistaRepository
    ):
        # Depende de abstracciones, no de implementaciones concretas
        self.pedido_repo = pedido_repo
        self.transportista_repo = transportista_repo

    def asignar_pedido_automaticamente(self, pedido_id: UUID) -> bool:
        """
        Caso de uso: Asignar automáticamente un pedido a un transportista disponible
        """
        # 1. Obtener el pedido
        pedido = self.pedido_repo.obtener_por_id(pedido_id)
        if not pedido or not pedido.is_pendiente():
            return False

        # 2. Buscar transportistas disponibles
        transportistas_disponibles = self.transportista_repo.obtener_disponibles()
        if not transportistas_disponibles:
            return False

        # 3. Seleccionar el primer transportista disponible (aquí iría lógica más compleja)
        transportista_seleccionado = transportistas_disponibles[0]

        # 4. Asignar el pedido
        pedido.asignar_transportista(transportista_seleccionado.id)
        
        # 5. Marcar transportista como en ruta
        transportista_seleccionado.marcar_en_ruta()

        # 6. Persistir cambios
        self.pedido_repo.guardar(pedido)
        # self.transportista_repo.guardar(transportista_seleccionado)  # Se implementaría

        return True

    def obtener_pedidos_cliente(self, cliente_id: UUID) -> list:
        """Obtiene todos los pedidos de un cliente específico"""
        return self.pedido_repo.obtener_por_cliente(cliente_id)

    def rastrear_pedido(self, numero_tracking: str) -> Optional[Pedido]:
        """Rastrea un pedido por su número de tracking"""
        return self.pedido_repo.obtener_por_numero_tracking(numero_tracking)