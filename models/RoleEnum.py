from enum import Enum
class RoleEnum(str, Enum):
    admin = "admin"
    operador = "operador"
    cliente = "cliente"
    transportista = "transportista"