from enum import Enum
class TipoClienteEnum(str, Enum):
    persona : str = "persona"
    empresa : str = "empresa"