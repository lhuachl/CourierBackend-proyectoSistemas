"""
Entidades de dominio puras - Sin dependencias externas
Estas entidades representan los conceptos centrales del negocio
"""

from dataclasses import dataclass
from typing import Optional, List
from uuid import UUID
from datetime import datetime
from decimal import Decimal
from enum import Enum

# Value Objects
@dataclass(frozen=True)
class Coordenada:
    latitud: float
    longitud: float
    
    def __post_init__(self):
        if not (-90 <= self.latitud <= 90):
            raise ValueError("Latitud debe estar entre -90 y 90")
        if not (-180 <= self.longitud <= 180):
            raise ValueError("Longitud debe estar entre -180 y 180")

@dataclass(frozen=True)
class Dinero:
    monto: Decimal
    moneda: str = "PEN"
    
    def __post_init__(self):
        if self.monto < 0:
            raise ValueError("El monto no puede ser negativo")

@dataclass(frozen=True)
class DocumentoIdentidad:
    numero: str
    tipo: str
    
    def __post_init__(self):
        if not self.numero.strip():
            raise ValueError("Número de documento no puede estar vacío")

# Domain Events
@dataclass(frozen=True)
class EventoDominio:
    fecha_evento: datetime
    agregado_id: UUID

@dataclass(frozen=True)
class PedidoCreado(EventoDominio):
    numero_tracking: str
    cliente_id: UUID

@dataclass(frozen=True)
class PedidoAsignado(EventoDominio):
    transportista_id: UUID
    numero_tracking: str

@dataclass(frozen=True)
class PedidoEntregado(EventoDominio):
    numero_tracking: str
    fecha_entrega: datetime