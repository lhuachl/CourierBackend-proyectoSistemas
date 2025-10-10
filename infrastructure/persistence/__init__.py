"""
Capa de infraestructura de persistencia
========================================

Este módulo implementa el Patrón de Doble Modelo:
- Entidades de Dominio (Pydantic): Agnósticas a tecnología
- Modelos de Persistencia (SQLAlchemy): Específicos de base de datos

Componentes:
- models/: Modelos SQLAlchemy (PedidoDB, etc.)
- mappers/: Conversores entre dominio y persistencia
- *_repository_sqlalchemy.py: Implementaciones de repositorios
"""

from .pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
from .pedido_service_example import PedidoService
from .models import PedidoDB
from .mappers import PedidoMapper

__all__ = [
    # Repositorios
    "PedidoRepositorySQLAlchemy",
    
    # Servicios
    "PedidoService",
    
    # Modelos de Persistencia
    "PedidoDB",
    
    # Mappers
    "PedidoMapper",
]