from dataclasses import dataclass
from typing import List, Optional
from uuid import UUID

from domain.repositories.pedido_repository import IPedidoRepository
from models.pedidos import Pedido


@dataclass
class CrearPedidoUseCase:
    repo: IPedidoRepository

    def execute(self, pedido: Pedido) -> Pedido:
        self.repo.guardar(pedido)
        return pedido


@dataclass
class ObtenerPedidoPorIdUseCase:
    repo: IPedidoRepository

    def execute(self, id: UUID) -> Optional[Pedido]:
        return self.repo.obtener_por_id(id)


@dataclass
class ObtenerPedidoPorTrackingUseCase:
    repo: IPedidoRepository

    def execute(self, numero_tracking: str) -> Optional[Pedido]:
        return self.repo.obtener_por_numero_tracking(numero_tracking)


@dataclass
class ListarPedidosPorClienteUseCase:
    repo: IPedidoRepository

    def execute(self, cliente_id: UUID) -> List[Pedido]:
        return self.repo.obtener_por_cliente(cliente_id)


@dataclass
class ListarPedidosPendientesUseCase:
    repo: IPedidoRepository

    def execute(self) -> List[Pedido]:
        return self.repo.obtener_pendientes()
