"""
Rutas CRUD para usuarios
"""
from fastapi import APIRouter, Depends, HTTPException, status
from typing import List
from uuid import UUID

from api.schemas import UserResponse, UserUpdate
from api.dependencies import get_user_repository, get_current_user, require_admin
from infrastructure.persistence.user_repository_sqlalchemy import UserRepositorySQLAlchemy
from models.usuarios import User
from models.enums import EstadoEnum

router = APIRouter(prefix="/users", tags=["Usuarios"])

@router.get("/me", response_model=UserResponse)
def get_current_user_info(current_user: User = Depends(get_current_user)):
    """
    Obtiene información del usuario actual
    """
    return UserResponse(
        id=str(current_user.id),
        email=current_user.email,
        nombre=current_user.nombre,
        apellido=current_user.apellido,
        role=current_user.role.value,
        estado=current_user.estado.value,
        created_at=current_user.created_at
    )

@router.get("/", response_model=List[UserResponse])
def list_users(
    skip: int = 0,
    limit: int = 100,
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository),
    admin_user: User = Depends(require_admin)
):
    """
    Lista todos los usuarios (solo admin)
    """
    # Implementación simplificada - en producción usar paginación real
    users = []
    # Por ahora retornamos lista vacía, se puede extender
    return users

@router.get("/{user_id}", response_model=UserResponse)
def get_user(
    user_id: UUID,
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Obtiene un usuario por ID
    
    Solo admin puede ver otros usuarios, usuarios normales solo pueden ver su propia info
    """
    # Si no es admin, solo puede ver su propia información
    if not current_user.is_admin() and current_user.id != user_id:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="No tienes permisos para ver este usuario"
        )
    
    user = user_repo.obtener_por_id(user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Usuario no encontrado"
        )
    
    return UserResponse(
        id=str(user.id),
        email=user.email,
        nombre=user.nombre,
        apellido=user.apellido,
        role=user.role.value,
        estado=user.estado.value,
        created_at=user.created_at
    )

@router.put("/{user_id}", response_model=UserResponse)
def update_user(
    user_id: UUID,
    user_update: UserUpdate,
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Actualiza un usuario
    
    Solo admin puede actualizar otros usuarios, usuarios normales solo pueden actualizar su propia info
    """
    # Si no es admin, solo puede actualizar su propia información
    if not current_user.is_admin() and current_user.id != user_id:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="No tienes permisos para actualizar este usuario"
        )
    
    user = user_repo.obtener_por_id(user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Usuario no encontrado"
        )
    
    # Actualizar campos
    if user_update.nombre:
        user.nombre = user_update.nombre
    if user_update.apellido:
        user.apellido = user_update.apellido
    if user_update.estado and current_user.is_admin():
        # Solo admin puede cambiar el estado
        try:
            user.estado = EstadoEnum(user_update.estado)
        except ValueError:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail=f"Estado inválido. Estados válidos: {[e.value for e in EstadoEnum]}"
            )
    
    user_repo.guardar(user)
    
    return UserResponse(
        id=str(user.id),
        email=user.email,
        nombre=user.nombre,
        apellido=user.apellido,
        role=user.role.value,
        estado=user.estado.value,
        created_at=user.created_at
    )

@router.delete("/{user_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_user(
    user_id: UUID,
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository),
    admin_user: User = Depends(require_admin)
):
    """
    Elimina un usuario (solo admin)
    """
    user = user_repo.obtener_por_id(user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Usuario no encontrado"
        )
    
    user_repo.eliminar(user_id)
    return None
