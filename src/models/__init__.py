# Models module for Courier System - Clean Architecture
# Exportaciones centralizadas organizadas por dominio

# Modelo base y mixins
from .base import BaseEntity, TimestampedEntity, StandartModel
from .base import GeoLocationMixin, ContactInfoMixin, AddressMixin

# Enums organizados por categoría
from .enums import (
    # Estados
    EstadoEnum, EstadoPedidoEnum, EstadoTransportistaEnum,
    EstadoTrackingEnum, EstadoPagoEnum, EstadoFacturaEnum, EstadoRutaEnum,
    
    # Tipos
    TipoClienteEnum, TipoVehiculoEnum, TipoDireccionEnum, MetodoPagoEnum,
    
    # Roles y Prioridades
    RoleEnum, PrioridadEnum,
)

# Modelos por dominio
from .usuarios import User
from .clientes import Cliente, Direccion
from .pedidos import Pedido, Factura, Pago
from .transportes import Transportista, Tracking, Ruta, DetalleRuta
from .geograficos import Zona
from .auxiliares import Calificacion, Notificacion

__all__ = [
    # Base y Mixins
    "BaseEntity",
    "TimestampedEntity", 
    "StandartModel",
    "GeoLocationMixin",
    "ContactInfoMixin",
    "AddressMixin",
    
    # Enums
    "EstadoEnum", "EstadoPedidoEnum", "EstadoTransportistaEnum",
    "EstadoTrackingEnum", "EstadoPagoEnum", "EstadoFacturaEnum", "EstadoRutaEnum",
    "TipoClienteEnum", "TipoVehiculoEnum", "TipoDireccionEnum", "MetodoPagoEnum",
    "RoleEnum", "PrioridadEnum",
    
    # Entidades por Dominio
    # Usuarios
    "User",
    
    # Clientes
    "Cliente", "Direccion",
    
    # Pedidos
    "Pedido", "Factura", "Pago",
    
    # Transportes
    "Transportista", "Tracking", "Ruta", "DetalleRuta",
    
    # Geográficos
    "Zona",
    
    # Auxiliares
    "Calificacion", "Notificacion",
]