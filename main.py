"""
Aplicación principal FastAPI para Sistema Courier
"""
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from api.routes import auth_router, users_router, pedidos_router
from infrastructure.persistence.models.pedido_db import Base as PedidoBase
from infrastructure.persistence.models.user_db import Base as UserBase
from api.dependencies import engine

# Crear aplicación FastAPI
app = FastAPI(
    title="Sistema Courier API",
    description="API RESTful para sistema de gestión de pedidos y entregas",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

# Configurar CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # En producción, especificar orígenes permitidos
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Crear tablas en la base de datos
PedidoBase.metadata.create_all(bind=engine)
UserBase.metadata.create_all(bind=engine)

# Incluir routers
app.include_router(auth_router)
app.include_router(users_router)
app.include_router(pedidos_router)

@app.get("/")
def root():
    """Endpoint raíz"""
    return {
        "message": "API Sistema Courier",
        "version": "1.0.0",
        "docs": "/docs"
    }

@app.get("/health")
def health_check():
    """Health check endpoint"""
    return {"status": "ok"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
