"""
Implementación concreta del repositorio de pedidos usando SQLAlchemy
Esta implementación está en la capa de infraestructura y depende de tecnologías específicas
"""

from typing import Optional, List
from uuid import UUID
from sqlalchemy.orm import Session

# Interfaces del dominio
from domain.repositories.pedido_repository import IPedidoRepository

# Entidades del modelo de dominio
from models.pedidos import Pedido

# Modelos de persistencia y mappers
from infrastructure.persistence.models.pedido_db import PedidoDB
from infrastructure.persistence.mappers.pedido_mapper import PedidoMapper
from models.enums import EstadoPedidoEnum

class PedidoRepositorySQLAlchemy(IPedidoRepository):
    """Implementación SQLAlchemy del repositorio de pedidos
    
    Esta clase implementa el patrón de doble modelo:
    - PedidoDB: Modelo SQLAlchemy para persistencia
    - Pedido: Entidad de dominio (Pydantic)
    - PedidoMapper: Convierte entre ambos modelos
    """
    
    def __init__(self, session: Session):
        self.session = session
        self.mapper = PedidoMapper()

    def guardar(self, entidad: Pedido) -> None:
        """Guarda un pedido en la base de datos
        
        Args:
            entidad: Entidad de dominio a persistir
        """
        pedido_db = self.mapper.to_persistence(entidad)
        self.session.merge(pedido_db)  # merge maneja insert/update
        self.session.commit()

    def obtener_por_id(self, id: UUID) -> Optional[Pedido]:
        """Obtiene un pedido por su ID
        
        Args:
            id: UUID del pedido
            
        Returns:
            Pedido de dominio o None si no existe
        """
        pedido_db = self.session.query(PedidoDB).filter(PedidoDB.id == id).first()
        return self.mapper.to_domain(pedido_db) if pedido_db else None

    def eliminar(self, id: UUID) -> None:
        """Elimina un pedido
        
        Args:
            id: UUID del pedido a eliminar
        """
        self.session.query(PedidoDB).filter(PedidoDB.id == id).delete()
        self.session.commit()

    def obtener_por_numero_tracking(self, numero_tracking: str) -> Optional[Pedido]:
        """Obtiene un pedido por su número de tracking
        
        Args:
            numero_tracking: Número de tracking único
            
        Returns:
            Pedido de dominio o None si no existe
        """
        pedido_db = self.session.query(PedidoDB).filter(
            PedidoDB.numero_tracking == numero_tracking
        ).first()
        return self.mapper.to_domain(pedido_db) if pedido_db else None

    def obtener_por_cliente(self, cliente_id: UUID) -> List[Pedido]:
        """Obtiene todos los pedidos de un cliente
        
        Args:
            cliente_id: UUID del cliente
            
        Returns:
            Lista de pedidos del cliente
        """
        pedidos_db = self.session.query(PedidoDB).filter(
            PedidoDB.id_cliente == cliente_id
        ).all()
        return [self.mapper.to_domain(p) for p in pedidos_db]

    def obtener_por_transportista(self, transportista_id: UUID) -> List[Pedido]:
        """Obtiene todos los pedidos asignados a un transportista
        
        Args:
            transportista_id: UUID del transportista
            
        Returns:
            Lista de pedidos del transportista
        """
        pedidos_db = self.session.query(PedidoDB).filter(
            PedidoDB.id_transportista == transportista_id
        ).all()
        return [self.mapper.to_domain(p) for p in pedidos_db]

    def obtener_pendientes(self) -> List[Pedido]:
        """Obtiene todos los pedidos pendientes
        
        Returns:
            Lista de pedidos con estado pendiente
        """
        pedidos_db = self.session.query(PedidoDB).filter(
            PedidoDB.estado == EstadoPedidoEnum.pendiente
        ).all()
        return [self.mapper.to_domain(p) for p in pedidos_db]

    def obtener_en_ruta(self) -> List[Pedido]:
        """Obtiene todos los pedidos en ruta
        
        Returns:
            Lista de pedidos con estado en_ruta
        """
        pedidos_db = self.session.query(PedidoDB).filter(
            PedidoDB.estado == EstadoPedidoEnum.en_ruta
        ).all()
        return [self.mapper.to_domain(p) for p in pedidos_db]