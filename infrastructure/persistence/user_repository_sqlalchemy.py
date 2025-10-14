"""
Implementación concreta del repositorio de usuarios usando SQLAlchemy
"""
from typing import Optional, List
from uuid import UUID
from sqlalchemy.orm import Session

# Interfaces del dominio
from domain.repositories.user_repository import IUserRepository

# Entidades del modelo de dominio
from models.usuarios import User

# Modelos de persistencia y mappers
from infrastructure.persistence.models.user_db import UserDB
from infrastructure.persistence.mappers.user_mapper import UserMapper
from models.enums import RoleEnum

class UserRepositorySQLAlchemy(IUserRepository):
    """Implementación SQLAlchemy del repositorio de usuarios
    
    Esta clase implementa el patrón de doble modelo:
    - UserDB: Modelo SQLAlchemy para persistencia
    - User: Entidad de dominio (Pydantic)
    - UserMapper: Convierte entre ambos modelos
    """
    
    def __init__(self, session: Session):
        self.session = session
        self.mapper = UserMapper()
    
    def guardar(self, entidad: User) -> None:
        """Guarda un usuario en la base de datos
        
        Args:
            entidad: Entidad de dominio a persistir
        """
        user_db = self.mapper.to_persistence(entidad)
        self.session.merge(user_db)  # merge maneja insert/update
        self.session.commit()
    
    def obtener_por_id(self, id: UUID) -> Optional[User]:
        """Obtiene un usuario por su ID
        
        Args:
            id: UUID del usuario
            
        Returns:
            User de dominio o None si no existe
        """
        user_db = self.session.query(UserDB).filter(UserDB.id == id).first()
        return self.mapper.to_domain(user_db) if user_db else None
    
    def eliminar(self, id: UUID) -> None:
        """Elimina un usuario
        
        Args:
            id: UUID del usuario a eliminar
        """
        self.session.query(UserDB).filter(UserDB.id == id).delete()
        self.session.commit()
    
    def obtener_por_email(self, email: str) -> Optional[User]:
        """Obtiene un usuario por su email
        
        Args:
            email: Email del usuario
            
        Returns:
            User de dominio o None si no existe
        """
        user_db = self.session.query(UserDB).filter(UserDB.email == email).first()
        return self.mapper.to_domain(user_db) if user_db else None
    
    def obtener_por_rol(self, rol: str) -> List[User]:
        """Obtiene usuarios por rol
        
        Args:
            rol: Rol a buscar
            
        Returns:
            Lista de usuarios con ese rol
        """
        users_db = self.session.query(UserDB).filter(UserDB.role == rol).all()
        return [self.mapper.to_domain(u) for u in users_db]
    
    def email_existe(self, email: str) -> bool:
        """Verifica si un email ya existe
        
        Args:
            email: Email a verificar
            
        Returns:
            True si el email existe, False en caso contrario
        """
        return self.session.query(UserDB).filter(UserDB.email == email).count() > 0
