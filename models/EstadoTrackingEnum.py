from enum import Enum

class EstadoTrackingEnum(str, Enum):
    recogido = "recogido"
    en_ruta = "en_ruta"
    entregado = "entregado"
    fallido = "fallido"