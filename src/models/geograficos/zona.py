from pydantic import Field

from ..base import TimestampedEntity

class Zona(TimestampedEntity):
    """Entidad Zona del dominio"""
    nombre: str = Field(..., description="Zone name")
    ciudad: str = Field(..., description="City name")
    poligono_geo: dict = Field(..., description="GeoJSON polygon representation")
    
    def contiene_punto(self, latitud: float, longitud: float) -> bool:
        """
        Verifica si un punto está dentro de la zona
        TODO: Implementar algoritmo de point-in-polygon
        """
        # Placeholder - se implementaría con algoritmo real
        return True
    
    @property
    def area_aproximada(self) -> float:
        """
        Calcula el área aproximada de la zona
        TODO: Implementar cálculo basado en GeoJSON
        """
        # Placeholder - se implementaría con cálculo real
        return 0.0