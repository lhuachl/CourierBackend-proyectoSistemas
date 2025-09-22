from enum import Enum

class EstadoPagoEnum(str, Enum):
    pendiente = "pendiente"
    completado = "completado"
    fallido = "fallido"