from enum import Enum

class EstadoPedidoEnum(str, Enum):
    pendiente = "pendiente"
    procesando = "procesando"
    en_ruta = "en_ruta"
    entregado = "entregado"
    cancelado = "cancelado"