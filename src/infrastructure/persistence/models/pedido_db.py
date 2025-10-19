"""Modelo SQLAlchemy para persistencia de pedidos"""
from sqlalchemy import Column, String, DateTime, Numeric, Enum as SQLEnum
from sqlalchemy.types import TypeDecorator, CHAR
from sqlalchemy.dialects.postgresql import UUID as PGUUID
from sqlalchemy.ext.declarative import declarative_base
from datetime import datetime
from uuid import uuid4
import uuid

from models.enums import EstadoPedidoEnum, PrioridadEnum

Base = declarative_base()

class GUID(TypeDecorator):
    """Platform-independent GUID type.
    Uses PostgreSQL's UUID type, otherwise uses CHAR(36), storing as stringified hex values.
    """
    impl = CHAR
    cache_ok = True

    def load_dialect_impl(self, dialect):
        if dialect.name == 'postgresql':
            return dialect.type_descriptor(PGUUID(as_uuid=True))
        else:
            return dialect.type_descriptor(CHAR(36))

    def process_bind_param(self, value, dialect):
        if value is None:
            return value
        elif dialect.name == 'postgresql':
            return str(value)
        else:
            if not isinstance(value, uuid.UUID):
                return str(uuid.UUID(value))
            else:
                return str(value)

    def process_result_value(self, value, dialect):
        if value is None:
            return value
        else:
            if not isinstance(value, uuid.UUID):
                return uuid.UUID(value)
            else:
                return value

class PedidoDB(Base):
    """Modelo SQLAlchemy para persistencia de pedidos"""
    __tablename__ = "pedidos"
    
    id = Column(GUID(), primary_key=True, default=uuid4)
    numero_tracking = Column(String, unique=True, nullable=False, index=True)
    id_cliente = Column(GUID(), nullable=False, index=True)
    fecha_solicitud = Column(DateTime, nullable=False)
    fecha_entrega_estimada = Column(DateTime, nullable=False)
    fecha_entrega_real = Column(DateTime, nullable=True)
    direccion_origen = Column(GUID(), nullable=False)
    direccion_destino = Column(GUID(), nullable=False)
    estado = Column(SQLEnum(EstadoPedidoEnum), nullable=False, index=True)
    prioridad = Column(SQLEnum(PrioridadEnum), nullable=False)
    peso = Column(Numeric(10, 2), nullable=False)
    dimensiones = Column(String, nullable=True)
    monto_total = Column(Numeric(10, 2), nullable=False)
    id_transportista = Column(GUID(), nullable=True, index=True)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow, nullable=False)
    
    def __repr__(self):
        return f"<PedidoDB(id={self.id}, numero_tracking={self.numero_tracking}, estado={self.estado})>"
