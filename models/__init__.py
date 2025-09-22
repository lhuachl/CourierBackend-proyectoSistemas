# Models module for Courier System
# Contains all Pydantic models for the MVP Courier application

from .StandartModel import StandartModel

# Enums
from .RoleEnum import RoleEnum
from .EstadoEnum import EstadoEnum
from .TipoClienteEnum import TipoClienteEnum
from .TipoVehiculoEnum import TipoVehiculoEnum
from .EstadoTransportistaEnum import EstadoTransportistaEnum
from .TipoDireccionEnum import TipoDireccionEnum
from .EstadoPedidoEnum import EstadoPedidoEnum
from .PrioridadEnum import PrioridadEnum
from .EstadoTrackingEnum import EstadoTrackingEnum
from .MetodoPagoEnum import MetodoPagoEnum
from .EstadoPagoEnum import EstadoPagoEnum
from .EstadoFacturaEnum import EstadoFacturaEnum
from .EstadoRutaEnum import EstadoRutaEnum

# Main Models
from .Users import User
from .Clientes import Cliente
from .Transportistas import Transportista
from .Direcciones import Direccion
from .Zonas import Zona
from .Pedidos import Pedido
from .Tracking import Tracking
from .Pagos import Pago
from .Facturas import Factura
from .Rutas import Ruta
from .DetalleRutas import DetalleRuta
from .Calificaciones import Calificacion
from .Notificaciones import Notificacion

__all__ = [
    # Base Model
    "StandartModel",
    
    # Enums
    "RoleEnum",
    "EstadoEnum", 
    "TipoClienteEnum",
    "TipoVehiculoEnum",
    "EstadoTransportistaEnum",
    "TipoDireccionEnum",
    "EstadoPedidoEnum",
    "PrioridadEnum",
    "EstadoTrackingEnum",
    "MetodoPagoEnum",
    "EstadoPagoEnum",
    "EstadoFacturaEnum",
    "EstadoRutaEnum",
    
    # Models
    "User",
    "Cliente",
    "Transportista",
    "Direccion",
    "Zona",
    "Pedido",
    "Tracking",
    "Pago",
    "Factura",
    "Ruta",
    "DetalleRuta",
    "Calificacion",
    "Notificacion",
]