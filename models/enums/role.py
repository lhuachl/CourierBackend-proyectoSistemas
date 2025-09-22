from enum import Enum

class RoleEnum(str, Enum):
    """Roles de usuario en el sistema"""
    admin = "admin"
    operador = "operador"
    cliente = "cliente"
    transportista = "transportista"