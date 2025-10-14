"""
Mapper bidireccional entre User (dominio) y UserDB (persistencia)
"""
from typing import Optional
from infrastructure.persistence.models.user_db import UserDB
from models.usuarios import User
from models.enums import RoleEnum, EstadoEnum

def get_enum_value(obj):
    """Helper para obtener valor de enum o string"""
    return obj.value if hasattr(obj, 'value') else obj

class UserMapper:
    """Convierte entre la entidad de dominio User y el modelo de persistencia UserDB"""
    
    @staticmethod
    def to_domain(user_db: UserDB) -> User:
        """Convierte de modelo de persistencia a entidad de dominio
        
        Args:
            user_db: Modelo SQLAlchemy
            
        Returns:
            Entidad de dominio User (Pydantic)
        """
        return User(
            id=user_db.id,  # type: ignore
            created_at=user_db.created_at,  # type: ignore
            updated_at=user_db.updated_at,  # type: ignore
            email=user_db.email,
            password_hash=user_db.password_hash,
            nombre=user_db.nombre,
            apellido=user_db.apellido,
            role=RoleEnum(user_db.role),
            estado=EstadoEnum(user_db.estado)
        )
    
    @staticmethod
    def to_persistence(user: User) -> UserDB:
        """Convierte de entidad de dominio a modelo de persistencia
        
        Args:
            user: Entidad de dominio
            
        Returns:
            Modelo SQLAlchemy UserDB
        """
        return UserDB(
            id=user.id,
            created_at=user.created_at,
            updated_at=user.updated_at,
            email=user.email,
            password_hash=user.password_hash,
            nombre=user.nombre,
            apellido=user.apellido,
            role=get_enum_value(user.role),
            estado=get_enum_value(user.estado)
        )
