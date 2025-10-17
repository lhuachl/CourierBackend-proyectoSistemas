# Routers package
from .health import router as health_router
from .pedidos import router as pedidos_router

__all__ = [
    "health_router",
    "pedidos_router",
]
