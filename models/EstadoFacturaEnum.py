from enum import Enum

class EstadoFacturaEnum(str, Enum):
    emitida = "emitida"
    pagada = "pagada"
    anulada = "anulada"