"""
Rutas CRUD para pedidos
"""
from fastapi import APIRouter, Depends, HTTPException, status
from typing import List
from uuid import UUID, uuid4
from datetime import datetime

from api.schemas import PedidoCreate, PedidoUpdate, PedidoResponse
from api.dependencies import get_pedido_repository, get_current_user
from api.utils import get_enum_value
from infrastructure.persistence.pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
from models.usuarios import User
from models.pedidos import Pedido
from models.enums import EstadoPedidoEnum, PrioridadEnum

router = APIRouter(prefix="/pedidos", tags=["Pedidos"])

@router.post("/", response_model=PedidoResponse, status_code=status.HTTP_201_CREATED)
def create_pedido(
    pedido_data: PedidoCreate,
    pedido_repo: PedidoRepositorySQLAlchemy = Depends(get_pedido_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Crea un nuevo pedido
    
    Solo clientes y admin pueden crear pedidos
    """
    if not current_user.can_create_orders():
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="No tienes permisos para crear pedidos"
        )
    
    # Validar prioridad
    try:
        prioridad = PrioridadEnum(pedido_data.prioridad)
    except ValueError:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Prioridad inválida. Prioridades válidas: {[p.value for p in PrioridadEnum]}"
        )
    
    # Crear pedido
    pedido = Pedido(
        id=uuid4(),
        numero_tracking=pedido_data.numero_tracking,
        id_cliente=UUID(pedido_data.id_cliente),
        fecha_solicitud=datetime.utcnow(),
        fecha_entrega_estimada=pedido_data.fecha_entrega_estimada,
        direccion_origen=UUID(pedido_data.direccion_origen),
        direccion_destino=UUID(pedido_data.direccion_destino),
        estado=EstadoPedidoEnum.pendiente,
        prioridad=prioridad,
        peso=pedido_data.peso,
        monto_total=pedido_data.monto_total
    )
    
    pedido_repo.guardar(pedido)
    
    return PedidoResponse(
        id=str(pedido.id),
        numero_tracking=pedido.numero_tracking,
        id_cliente=str(pedido.id_cliente),
        estado=get_enum_value(pedido.estado),
        prioridad=get_enum_value(pedido.prioridad),
        peso=pedido.peso,
        monto_total=pedido.monto_total,
        fecha_solicitud=pedido.fecha_solicitud,
        fecha_entrega_estimada=pedido.fecha_entrega_estimada,
        id_transportista=str(pedido.id_transportista) if pedido.id_transportista else None
    )

@router.get("/{pedido_id}", response_model=PedidoResponse)
def get_pedido(
    pedido_id: UUID,
    pedido_repo: PedidoRepositorySQLAlchemy = Depends(get_pedido_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Obtiene un pedido por ID
    """
    pedido = pedido_repo.obtener_por_id(pedido_id)
    if not pedido:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Pedido no encontrado"
        )
    
    # Verificar permisos: solo el cliente dueño, transportista asignado o admin pueden ver
    can_view = (
        current_user.is_admin() or
        pedido.id_cliente == current_user.id or
        (pedido.id_transportista and pedido.id_transportista == current_user.id)
    )
    
    if not can_view:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="No tienes permisos para ver este pedido"
        )
    
    return PedidoResponse(
        id=str(pedido.id),
        numero_tracking=pedido.numero_tracking,
        id_cliente=str(pedido.id_cliente),
        estado=get_enum_value(pedido.estado),
        prioridad=get_enum_value(pedido.prioridad),
        peso=pedido.peso,
        monto_total=pedido.monto_total,
        fecha_solicitud=pedido.fecha_solicitud,
        fecha_entrega_estimada=pedido.fecha_entrega_estimada,
        id_transportista=str(pedido.id_transportista) if pedido.id_transportista else None
    )

@router.get("/tracking/{numero_tracking}", response_model=PedidoResponse)
def get_pedido_by_tracking(
    numero_tracking: str,
    pedido_repo: PedidoRepositorySQLAlchemy = Depends(get_pedido_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Obtiene un pedido por número de tracking
    """
    pedido = pedido_repo.obtener_por_numero_tracking(numero_tracking)
    if not pedido:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Pedido no encontrado"
        )
    
    return PedidoResponse(
        id=str(pedido.id),
        numero_tracking=pedido.numero_tracking,
        id_cliente=str(pedido.id_cliente),
        estado=get_enum_value(pedido.estado),
        prioridad=get_enum_value(pedido.prioridad),
        peso=pedido.peso,
        monto_total=pedido.monto_total,
        fecha_solicitud=pedido.fecha_solicitud,
        fecha_entrega_estimada=pedido.fecha_entrega_estimada,
        id_transportista=str(pedido.id_transportista) if pedido.id_transportista else None
    )

@router.get("/", response_model=List[PedidoResponse])
def list_pedidos(
    skip: int = 0,
    limit: int = 100,
    pedido_repo: PedidoRepositorySQLAlchemy = Depends(get_pedido_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Lista pedidos según el rol del usuario
    
    - Admin: todos los pedidos
    - Cliente: sus propios pedidos
    - Transportista: pedidos asignados
    """
    if current_user.is_admin():
        # Admin ve pedidos pendientes por defecto
        pedidos = pedido_repo.obtener_pendientes()
    elif get_enum_value(current_user.role) == "cliente":
        pedidos = pedido_repo.obtener_por_cliente(current_user.id)
    elif get_enum_value(current_user.role) == "transportista":
        pedidos = pedido_repo.obtener_por_transportista(current_user.id)
    else:
        pedidos = []
    
    return [
        PedidoResponse(
            id=str(p.id),
            numero_tracking=p.numero_tracking,
            id_cliente=str(p.id_cliente),
            estado=get_enum_value(p.estado),
            prioridad=get_enum_value(p.prioridad),
            peso=p.peso,
            monto_total=p.monto_total,
            fecha_solicitud=p.fecha_solicitud,
            fecha_entrega_estimada=p.fecha_entrega_estimada,
            id_transportista=str(p.id_transportista) if p.id_transportista else None
        )
        for p in pedidos[skip:skip+limit]
    ]

@router.put("/{pedido_id}", response_model=PedidoResponse)
def update_pedido(
    pedido_id: UUID,
    pedido_update: PedidoUpdate,
    pedido_repo: PedidoRepositorySQLAlchemy = Depends(get_pedido_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Actualiza un pedido
    
    - Admin puede actualizar todo
    - Transportista puede actualizar estado
    """
    pedido = pedido_repo.obtener_por_id(pedido_id)
    if not pedido:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Pedido no encontrado"
        )
    
    # Verificar permisos
    can_update = (
        current_user.is_admin() or
        (current_user.can_deliver_orders() and pedido.id_transportista == current_user.id)
    )
    
    if not can_update:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="No tienes permisos para actualizar este pedido"
        )
    
    # Actualizar campos
    if pedido_update.estado:
        try:
            nuevo_estado = EstadoPedidoEnum(pedido_update.estado)
            pedido.estado = nuevo_estado
        except ValueError:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail=f"Estado inválido. Estados válidos: {[e.value for e in EstadoPedidoEnum]}"
            )
    
    if pedido_update.id_transportista and current_user.is_admin():
        pedido.asignar_transportista(UUID(pedido_update.id_transportista))
    
    if pedido_update.fecha_entrega_estimada and current_user.is_admin():
        pedido.fecha_entrega_estimada = pedido_update.fecha_entrega_estimada
    
    pedido_repo.guardar(pedido)
    
    return PedidoResponse(
        id=str(pedido.id),
        numero_tracking=pedido.numero_tracking,
        id_cliente=str(pedido.id_cliente),
        estado=get_enum_value(pedido.estado),
        prioridad=get_enum_value(pedido.prioridad),
        peso=pedido.peso,
        monto_total=pedido.monto_total,
        fecha_solicitud=pedido.fecha_solicitud,
        fecha_entrega_estimada=pedido.fecha_entrega_estimada,
        id_transportista=str(pedido.id_transportista) if pedido.id_transportista else None
    )

@router.delete("/{pedido_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_pedido(
    pedido_id: UUID,
    pedido_repo: PedidoRepositorySQLAlchemy = Depends(get_pedido_repository),
    current_user: User = Depends(get_current_user)
):
    """
    Elimina un pedido (solo admin)
    """
    if not current_user.is_admin():
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Solo admin puede eliminar pedidos"
        )
    
    pedido = pedido_repo.obtener_por_id(pedido_id)
    if not pedido:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Pedido no encontrado"
        )
    
    pedido_repo.eliminar(pedido_id)
    return None
