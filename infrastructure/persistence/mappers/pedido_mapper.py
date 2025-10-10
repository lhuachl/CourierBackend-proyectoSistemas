"""Mapper para conversión entre entidad Pedido y modelo PedidoDB"""
from typing import Optional
from ....models.pedidos import Pedido
from ..models.pedido_db import PedidoDB

class PedidoMapper:
    """Convierte entre entidad de dominio y modelo de persistencia"""
    
    @staticmethod
    def to_domain(pedido_db: PedidoDB) -> Pedido:
        """Convierte de SQLAlchemy a Pydantic/dominio
        
        Args:
            pedido_db: Modelo SQLAlchemy de pedido
            
        Returns:
            Pedido: Entidad de dominio
            
        Note:
            Los atributos de pedido_db son instancias de Column en tiempo de definición,
            pero en tiempo de ejecución SQLAlchemy los convierte a los valores reales.
            Por eso usamos # type: ignore para evitar warnings de Pylance.
        """
        return Pedido(
            id=pedido_db.id,  # type: ignore
            numero_tracking=pedido_db.numero_tracking,  # type: ignore
            id_cliente=pedido_db.id_cliente,  # type: ignore
            fecha_solicitud=pedido_db.fecha_solicitud,  # type: ignore
            fecha_entrega_estimada=pedido_db.fecha_entrega_estimada,  # type: ignore
            fecha_entrega_real=pedido_db.fecha_entrega_real,  # type: ignore
            direccion_origen=pedido_db.direccion_origen,  # type: ignore
            direccion_destino=pedido_db.direccion_destino,  # type: ignore
            estado=pedido_db.estado,  # type: ignore
            prioridad=pedido_db.prioridad,  # type: ignore
            peso=pedido_db.peso,  # type: ignore
            dimensiones=pedido_db.dimensiones,  # type: ignore
            monto_total=pedido_db.monto_total,  # type: ignore
            id_transportista=pedido_db.id_transportista,  # type: ignore
            created_at=pedido_db.created_at,  # type: ignore
            updated_at=pedido_db.updated_at  # type: ignore
        )
    
    @staticmethod
    def to_persistence(pedido: Pedido) -> PedidoDB:
        """Convierte de Pydantic/dominio a SQLAlchemy
        
        Args:
            pedido: Entidad de dominio
            
        Returns:
            PedidoDB: Modelo SQLAlchemy
        """
        return PedidoDB(
            id=pedido.id,
            numero_tracking=pedido.numero_tracking,
            id_cliente=pedido.id_cliente,
            fecha_solicitud=pedido.fecha_solicitud,
            fecha_entrega_estimada=pedido.fecha_entrega_estimada,
            fecha_entrega_real=pedido.fecha_entrega_real,
            direccion_origen=pedido.direccion_origen,
            direccion_destino=pedido.direccion_destino,
            estado=pedido.estado,
            prioridad=pedido.prioridad,
            peso=pedido.peso,
            dimensiones=pedido.dimensiones,
            monto_total=pedido.monto_total,
            id_transportista=pedido.id_transportista,
            created_at=pedido.created_at,
            updated_at=pedido.updated_at
        )
    
    @staticmethod
    def update_persistence_from_domain(pedido_db: PedidoDB, pedido: Pedido) -> None:
        """Actualiza un modelo de persistencia existente con datos de la entidad de dominio
        
        Args:
            pedido_db: Modelo SQLAlchemy a actualizar
            pedido: Entidad de dominio con los nuevos datos
            
        Note:
            Los type: ignore son necesarios porque Pylance ve los atributos como Column,
            pero SQLAlchemy permite asignar valores directamente en instancias.
        """
        pedido_db.numero_tracking = pedido.numero_tracking  # type: ignore
        pedido_db.id_cliente = pedido.id_cliente  # type: ignore
        pedido_db.fecha_solicitud = pedido.fecha_solicitud  # type: ignore
        pedido_db.fecha_entrega_estimada = pedido.fecha_entrega_estimada  # type: ignore
        pedido_db.fecha_entrega_real = pedido.fecha_entrega_real  # type: ignore
        pedido_db.direccion_origen = pedido.direccion_origen  # type: ignore
        pedido_db.direccion_destino = pedido.direccion_destino  # type: ignore
        pedido_db.estado = pedido.estado  # type: ignore
        pedido_db.prioridad = pedido.prioridad  # type: ignore
        pedido_db.peso = pedido.peso  # type: ignore
        pedido_db.dimensiones = pedido.dimensiones  # type: ignore
        pedido_db.monto_total = pedido.monto_total  # type: ignore
        pedido_db.id_transportista = pedido.id_transportista  # type: ignore
        pedido_db.updated_at = pedido.updated_at  # type: ignore
