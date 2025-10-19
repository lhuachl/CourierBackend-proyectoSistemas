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
from .user_repository_sqlalchemy import UserRepositorySQLAlchemy
from .pedido_service_example import PedidoService
from .models import PedidoDB, UserDB
from .mappers import PedidoMapper, UserMapper

__all__ = [
    # Repositorios
    "PedidoRepositorySQLAlchemy",
    "UserRepositorySQLAlchemy",
    
    # Servicios
    "PedidoService",
    
    # Modelos de Persistencia
    "PedidoDB",
    "UserDB",
    
    # Mappers
    "PedidoMapper",
    "UserMapper",
]