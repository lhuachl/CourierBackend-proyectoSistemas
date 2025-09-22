from enum import Enum

class EstadoTransportistaEnum(str, Enum):
    disponible = "disponible"
    en_ruta = "en_ruta"
    inactivo = "inactivo"