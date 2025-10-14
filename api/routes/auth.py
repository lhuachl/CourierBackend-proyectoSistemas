"""
Rutas de autenticación
"""
from fastapi import APIRouter, Depends, HTTPException, status
from datetime import timedelta
from uuid import uuid4

from api.schemas import LoginRequest, RegisterRequest, TokenResponse
from api.dependencies import get_user_repository
from api.utils import get_enum_value
from infrastructure.auth import (
    verify_password,
    get_password_hash,
    create_access_token,
    ACCESS_TOKEN_EXPIRE_MINUTES
)
from infrastructure.persistence.user_repository_sqlalchemy import UserRepositorySQLAlchemy
from models.usuarios import User
from models.enums import RoleEnum, EstadoEnum

router = APIRouter(prefix="/auth", tags=["Autenticación"])

@router.post("/login", response_model=TokenResponse)
def login(
    login_data: LoginRequest,
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository)
):
    """
    Endpoint de login
    
    Autentica un usuario y devuelve un token JWT
    """
    # Buscar usuario por email
    user = user_repo.obtener_por_email(login_data.email)
    
    if not user:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Credenciales incorrectas"
        )
    
    # Verificar contraseña
    if not verify_password(login_data.password, user.password_hash):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Credenciales incorrectas"
        )
    
    # Verificar que el usuario esté activo
    if not user.is_active():
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Usuario inactivo"
        )
    
    # Crear token
    access_token_expires = timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    role_value = user.role.value if hasattr(user.role, 'value') else user.role
    access_token = create_access_token(
        data={"sub": str(user.id), "email": user.email, "role": role_value},
        expires_delta=access_token_expires
    )
    
    return TokenResponse(
        access_token=access_token,
        token_type="bearer",
        user_id=str(user.id),
        email=user.email,
        role=role_value
    )

@router.post("/register", response_model=TokenResponse, status_code=status.HTTP_201_CREATED)
def register(
    register_data: RegisterRequest,
    user_repo: UserRepositorySQLAlchemy = Depends(get_user_repository)
):
    """
    Endpoint de registro
    
    Crea un nuevo usuario y devuelve un token JWT
    """
    # Verificar que el email no exista
    if user_repo.email_existe(register_data.email):
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="El email ya está registrado"
        )
    
    # Validar rol
    try:
        role = RoleEnum(register_data.role)
    except ValueError:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Rol inválido. Roles válidos: {[r.value for r in RoleEnum]}"
        )
    
    # Crear usuario
    user = User(
        id=uuid4(),
        email=register_data.email,
        password_hash=get_password_hash(register_data.password),
        nombre=register_data.nombre,
        apellido=register_data.apellido,
        role=role,
        estado=EstadoEnum.activo
    )
    
    # Guardar en base de datos
    user_repo.guardar(user)
    
    # Crear token
    access_token_expires = timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)
    role_value = get_enum_value(user.role)
    access_token = create_access_token(
        data={"sub": str(user.id), "email": user.email, "role": role_value},
        expires_delta=access_token_expires
    )
    
    return TokenResponse(
        access_token=access_token,
        token_type="bearer",
        user_id=str(user.id),
        email=user.email,
        role=role_value
    )
