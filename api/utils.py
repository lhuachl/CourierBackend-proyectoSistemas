"""
Utilidades para la API
"""

def get_enum_value(obj):
    """Obtiene el valor de un enum o retorna el string directamente
    
    Args:
        obj: Enum o string
        
    Returns:
        String value
    """
    return obj.value if hasattr(obj, 'value') else obj
