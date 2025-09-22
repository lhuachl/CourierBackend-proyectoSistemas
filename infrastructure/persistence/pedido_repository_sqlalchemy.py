"""
Implementación concreta del repositorio de pedidos usando SQLAlchemy
Esta implementación está en la capa de infraestructura y depende de tecnologías específicas
"""

from typing import Optional, List
from uuid import UUID
from sqlalchemy.orm import Session

# Interfaces del dominio
from ...domain.repositories.pedido_repository import IPedidoRepository

# Entidades del modelo
from ...models.pedidos import Pedido

class PedidoRepositorySQLAlchemy(IPedidoRepository):
    """Implementación SQLAlchemy del repositorio de pedidos"""
    
    def __init__(self, session: Session):
        self.session = session

    def guardar(self, pedido: Pedido) -> None:
        """Guarda un pedido en la base de datos"""
        # Aquí se convertiría de entidad de dominio a modelo de datos si fuera necesario
        self.session.add(pedido)
        self.session.commit()

    def obtener_por_id(self, id: UUID) -> Optional[Pedido]:
        """Obtiene un pedido por su ID"""
        return self.session.query(Pedido).filter(Pedido.id == id).first()

    def eliminar(self, id: UUID) -> None:
        """Elimina un pedido"""
        pedido = self.obtener_por_id(id)
        if pedido:
            self.session.delete(pedido)
            self.session.commit()

    def obtener_por_numero_tracking(self, numero_tracking: str) -> Optional[Pedido]:
        """Obtiene un pedido por su número de tracking"""
        return self.session.query(Pedido).filter(
            Pedido.numero_tracking == numero_tracking
        ).first()

    def obtener_por_cliente(self, cliente_id: UUID) -> List[Pedido]:
        """Obtiene todos los pedidos de un cliente"""
        return self.session.query(Pedido).filter(
            Pedido.id_cliente == cliente_id
        ).all()

    def obtener_por_transportista(self, transportista_id: UUID) -> List[Pedido]:
        """Obtiene todos los pedidos asignados a un transportista"""
        return self.session.query(Pedido).filter(
            Pedido.id_transportista == transportista_id
        ).all()

    def obtener_pendientes(self) -> List[Pedido]:
        """Obtiene todos los pedidos pendientes"""
        from ...models.enums import EstadoPedidoEnum
        return self.session.query(Pedido).filter(
            Pedido.estado == EstadoPedidoEnum.pendiente
        ).all()

    def obtener_en_ruta(self) -> List[Pedido]:
        """Obtiene todos los pedidos en ruta"""
        from ...models.enums import EstadoPedidoEnum
        return self.session.query(Pedido).filter(
            Pedido.estado == EstadoPedidoEnum.en_ruta
        ).all()