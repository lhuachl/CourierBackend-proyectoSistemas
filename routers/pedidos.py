from uuid import UUID

from fastapi import APIRouter, HTTPException

from schemas.pedido import PedidoCreate, PedidoResponse

router = APIRouter(prefix="/pedidos", tags=["pedidos"]) 


@router.post("/", response_model=PedidoResponse, summary="Crear pedido")
def crear_pedido(payload: PedidoCreate):
    # TODO: Wire CrearPedidoUseCase con repositorio real
    raise HTTPException(status_code=501, detail="Crear pedido no implementado")


@router.get("/{pedido_id}", response_model=PedidoResponse, summary="Obtener pedido por ID")
def obtener_pedido(pedido_id: UUID):
    # TODO: Wire ObtenerPedidoPorIdUseCase
    raise HTTPException(status_code=501, detail="Obtener pedido no implementado")


@router.get("/tracking/{numero_tracking}", response_model=PedidoResponse, summary="Obtener pedido por tracking")
def obtener_por_tracking(numero_tracking: str):
    # TODO: Wire ObtenerPedidoPorTrackingUseCase
    raise HTTPException(status_code=501, detail="Obtener por tracking no implementado")
