from enum import Enum
class EstadoEnum(str, Enum):
    activo = "activo"
    inactivo = "inactivo"
    suspendido = "suspendido"