# Interfaces de repositorios

from .base_repository import IRepository
from .pedido_repository import IPedidoRepository
from .user_repository import IUserRepository
from .transportista_repository import ITransportistaRepository

__all__ = [
    "IRepository",
    "IPedidoRepository",
    "IUserRepository", 
    "ITransportistaRepository",
]