from enum import Enum

class TipoClienteEnum(str, Enum):
    """Tipos de cliente"""
    persona = "persona"
    empresa = "empresa"

class TipoVehiculoEnum(str, Enum):
    """Tipos de vehículo para transportistas"""
    moto = "moto"
    auto = "auto"
    camioneta = "camioneta"
    camion = "camion"

class TipoDireccionEnum(str, Enum):
    """Tipos de dirección"""
    residencial = "residencial"
    comercial = "comercial"

class MetodoPagoEnum(str, Enum):
    """Métodos de pago disponibles"""
    efectivo = "efectivo"
    tarjeta = "tarjeta"
    transferencia = "transferencia"