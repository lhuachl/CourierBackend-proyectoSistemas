from enum import Enum

class PrioridadEnum(str, Enum):
    """Prioridades de pedidos"""
    normal = "normal"
    express = "express"