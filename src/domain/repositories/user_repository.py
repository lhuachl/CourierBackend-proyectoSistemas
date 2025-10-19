from abc import abstractmethod
from typing import Optional, List
from uuid import UUID
from .base_repository import IRepository

from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from ...models.usuarios import User

class IUserRepository(IRepository):
    """Interfaz para el repositorio de usuarios"""
    
    @abstractmethod
    def obtener_por_email(self, email: str) -> Optional['User']:
        """Obtiene un usuario por su email"""
        pass
    
    @abstractmethod
    def obtener_por_rol(self, rol: str) -> List['User']:
        """Obtiene usuarios por rol"""
        pass
    
    @abstractmethod
    def email_existe(self, email: str) -> bool:
        """Verifica si un email ya existe"""
        pass