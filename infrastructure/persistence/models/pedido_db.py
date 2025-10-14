"""Modelo SQLAlchemy para persistencia de pedidos"""
from sqlalchemy import Column, String, DateTime, Numeric, Enum as SQLEnum
from sqlalchemy.dialects.postgresql import UUID as PGUUID
from sqlalchemy.ext.declarative import declarative_base
from datetime import datetime
from uuid import uuid4

from models.enums import EstadoPedidoEnum, PrioridadEnum

Base = declarative_base()

class PedidoDB(Base):
    """Modelo SQLAlchemy para persistencia de pedidos"""
    __tablename__ = "pedidos"
    
    id = Column(PGUUID(as_uuid=True), primary_key=True, default=uuid4)
    numero_tracking = Column(String, unique=True, nullable=False, index=True)
    id_cliente = Column(PGUUID(as_uuid=True), nullable=False, index=True)
    fecha_solicitud = Column(DateTime, nullable=False)
    fecha_entrega_estimada = Column(DateTime, nullable=False)
    fecha_entrega_real = Column(DateTime, nullable=True)
    direccion_origen = Column(PGUUID(as_uuid=True), nullable=False)
    direccion_destino = Column(PGUUID(as_uuid=True), nullable=False)
    estado = Column(SQLEnum(EstadoPedidoEnum), nullable=False, index=True)
    prioridad = Column(SQLEnum(PrioridadEnum), nullable=False)
    peso = Column(Numeric(10, 2), nullable=False)
    dimensiones = Column(String, nullable=True)
    monto_total = Column(Numeric(10, 2), nullable=False)
    id_transportista = Column(PGUUID(as_uuid=True), nullable=True, index=True)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow, nullable=False)
    
    def __repr__(self):
        return f"<PedidoDB(id={self.id}, numero_tracking={self.numero_tracking}, estado={self.estado})>"
