from enum import Enum

class EstadoEnum(str, Enum):
    """Estados generales del sistema"""
    activo = "activo"
    inactivo = "inactivo"
    suspendido = "suspendido"

class EstadoPedidoEnum(str, Enum):
    """Estados específicos de pedidos"""
    pendiente = "pendiente"
    procesando = "procesando"
    en_ruta = "en_ruta"
    entregado = "entregado"
    cancelado = "cancelado"

class EstadoTransportistaEnum(str, Enum):
    """Estados específicos de transportistas"""
    disponible = "disponible"
    en_ruta = "en_ruta"
    inactivo = "inactivo"

class EstadoTrackingEnum(str, Enum):
    """Estados de seguimiento de pedidos"""
    recogido = "recogido"
    en_ruta = "en_ruta"
    entregado = "entregado"
    fallido = "fallido"

class EstadoPagoEnum(str, Enum):
    """Estados de pagos"""
    pendiente = "pendiente"
    completado = "completado"
    fallido = "fallido"

class EstadoFacturaEnum(str, Enum):
    """Estados de facturas"""
    emitida = "emitida"
    pagada = "pagada"
    anulada = "anulada"

class EstadoRutaEnum(str, Enum):
    """Estados de rutas"""
    pendiente = "pendiente"
    en_progreso = "en_progreso"
    completada = "completada"