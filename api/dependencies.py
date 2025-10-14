"""
Dependencias de FastAPI para inyección
"""
import os
from dotenv import load_dotenv
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from typing import Generator, Optional
from uuid import UUID

from infrastructure.auth import decode_access_token
from infrastructure.persistence.user_repository_sqlalchemy import UserRepositorySQLAlchemy
from infrastructure.persistence.pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
from models.usuarios import User

# Cargar variables de entorno desde .env
load_dotenv()

# Configuración de base de datos desde variables de entorno
DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./courier.db")

# Configurar engine según el tipo de base de datos
if DATABASE_URL.startswith("sqlite"):
    engine = create_engine(DATABASE_URL, connect_args={"check_same_thread": False})
else:
    engine = create_engine(DATABASE_URL)

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

# Esquema de seguridad
security = HTTPBearer()

def get_db() -> Generator[Session, None, None]:
    """Dependencia para obtener sesión de base de datos"""
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

def get_user_repository(db: Session = Depends(get_db)) -> UserRepositorySQLAlchemy:
    """Dependencia para obtener repositorio de usuarios"""
    return UserRepositorySQLAlchemy(db)

def get_pedido_repository(db: Session = Depends(get_db)) -> PedidoRepositorySQLAlchemy:
    """Dependencia para obtener repositorio de pedidos"""
    return PedidoRepositorySQLAlchemy(db)

def get_current_user(
    credentials: HTTPAuthorizationCredentials = Depends(security),
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository)
) -> User:
    """Dependencia para obtener el usuario actual autenticado
    
    Args:
        credentials: Credenciales del token Bearer
        user_repo: Repositorio de usuarios
        
    Returns:
        Usuario autenticado
        
    Raises:
        HTTPException: Si el token es inválido o el usuario no existe
    """
    token = credentials.credentials
    payload = decode_access_token(token)
    
    if payload is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Token inválido o expirado"
        )
    
    user_id_str: Optional[str] = payload.get("sub")
    if user_id_str is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Token inválido"
        )
    
    try:
        user_id = UUID(user_id_str)
    except ValueError:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="ID de usuario inválido"
        )
    
    user = user_repo.obtener_por_id(user_id)
    if user is None:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Usuario no encontrado"
        )
    
    if not user.is_active():
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Usuario inactivo"
        )
    
    return user

def require_admin(current_user: User = Depends(get_current_user)) -> User:
    """Dependencia que requiere que el usuario sea administrador
    
    Args:
        current_user: Usuario actual
        
    Returns:
        Usuario si es admin
        
    Raises:
        HTTPException: Si el usuario no es admin
    """
    if not current_user.is_admin():
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Se requieren permisos de administrador"
        )
    return current_user
