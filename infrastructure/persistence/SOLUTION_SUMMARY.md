# ✅ Patrón de Doble Modelo - Implementación Completa

## 🎯 Problema Resuelto

**ANTES:** ❌
```python
# Pylance error: Pedido no tiene columnas SQLAlchemy
self.session.query(Pedido).filter(Pedido.estado == EstadoPedidoEnum.pendiente)
```

**AHORA:** ✅
```python
# Sin errores: PedidoDB tiene columnas SQLAlchemy
self.session.query(PedidoDB).filter(PedidoDB.estado == EstadoPedidoEnum.pendiente)
```

---

## 📁 Archivos Creados

### 1. Modelo de Persistencia
📄 `infrastructure/persistence/models/pedido_db.py`
```python
class PedidoDB(Base):
    """Modelo SQLAlchemy con columnas reales"""
    __tablename__ = "pedidos"
    
    id = Column(PGUUID(as_uuid=True), primary_key=True)
    numero_tracking = Column(String, unique=True, index=True)
    estado = Column(SQLEnum(EstadoPedidoEnum), index=True)
    # ... más columnas
```

### 2. Mapper (Convertidor)
📄 `infrastructure/persistence/mappers/pedido_mapper.py`
```python
class PedidoMapper:
    @staticmethod
    def to_domain(pedido_db: PedidoDB) -> Pedido:
        """SQLAlchemy → Pydantic"""
        return Pedido(...)
    
    @staticmethod
    def to_persistence(pedido: Pedido) -> PedidoDB:
        """Pydantic → SQLAlchemy"""
        return PedidoDB(...)
```

### 3. Repositorio Actualizado
📄 `infrastructure/persistence/pedido_repository_sqlalchemy.py`
```python
class PedidoRepositorySQLAlchemy(IPedidoRepository):
    def __init__(self, session: Session):
        self.session = session
        self.mapper = PedidoMapper()  # ← Nuevo
    
    def obtener_por_id(self, id: UUID) -> Optional[Pedido]:
        # 1. Query con PedidoDB (SQLAlchemy) ✅
        pedido_db = self.session.query(PedidoDB).filter(
            PedidoDB.id == id
        ).first()
        
        # 2. Convertir a Pedido (Pydantic) ✅
        return self.mapper.to_domain(pedido_db) if pedido_db else None
```

### 4. Documentación
📄 `infrastructure/persistence/README.md`
- Explicación completa del patrón
- Diagramas de flujo
- Guía de troubleshooting

### 5. Ejemplos de Uso
📄 `infrastructure/persistence/example_usage.py`
- CRUD completo
- Queries complejas
- Uso de lógica de negocio

---

## 🔄 Flujo Completo

```
┌─────────────────────────────────────────────────────────────┐
│                    TU APLICACIÓN                             │
│              (FastAPI, Services, etc.)                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                 ENTIDAD DE DOMINIO                           │
│              Pedido (Pydantic Model)                         │
│                                                              │
│  - Agnóstico a tecnología                                   │
│  - Lógica de negocio pura                                   │
│  - Validación Pydantic                                       │
│  - Fácil de testear                                          │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                    MAPPER                                    │
│              PedidoMapper                                    │
│                                                              │
│  to_domain()       ←  Convierte  →      to_persistence()    │
│  (DB → Dominio)                         (Dominio → DB)      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│              MODELO DE PERSISTENCIA                          │
│              PedidoDB (SQLAlchemy Model)                     │
│                                                              │
│  - Declarative Base                                          │
│  - Columnas con tipos SQL                                    │
│  - Índices y constraints                                     │
│  - Relaciones ORM                                            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                  BASE DE DATOS                               │
│              PostgreSQL / MySQL / etc.                       │
└─────────────────────────────────────────────────────────────┘
```

---

## ✨ Ventajas Inmediatas

### 1. ✅ Sin Errores de Pylance
```python
# Queries usan PedidoDB (tiene columnas SQLAlchemy)
pedidos_db = self.session.query(PedidoDB).filter(
    PedidoDB.estado == EstadoPedidoEnum.pendiente  # ← No error!
).all()

# Devuelve Pedido (Pydantic)
return [self.mapper.to_domain(p) for p in pedidos_db]
```

### 2. 🔒 Separación de Responsabilidades
- **Pedido (Pydantic)**: Lógica de negocio
- **PedidoDB (SQLAlchemy)**: Persistencia
- **PedidoMapper**: Conversión

### 3. 🧪 Testeable
```python
# Test de dominio (sin BD)
def test_asignar_transportista():
    pedido = Pedido(...)
    pedido.asignar_transportista(uuid4())
    assert pedido.estado == EstadoPedidoEnum.procesando

# Test de persistencia (mock del mapper)
def test_guardar(mock_mapper):
    repo = PedidoRepositorySQLAlchemy(session)
    repo.mapper = mock_mapper
    # ...
```

### 4. 🔄 Flexibilidad
- Cambiar de PostgreSQL a MySQL: Solo cambias `PedidoDB`
- Agregar validaciones: Solo modificas `Pedido`
- Cambiar ORM: Solo reescribes mapper y modelos DB

---

## 📖 Cómo Usar

### Ejemplo 1: Crear y guardar pedido
```python
from infrastructure.persistence import PedidoRepositorySQLAlchemy
from models.pedidos import Pedido

# 1. Crear repositorio
repo = PedidoRepositorySQLAlchemy(session)

# 2. Crear entidad de dominio (Pydantic)
pedido = Pedido(
    numero_tracking="TRK-001",
    id_cliente=uuid4(),
    # ...
)

# 3. Guardar (mapper convierte automáticamente)
repo.guardar(pedido)
```

### Ejemplo 2: Buscar y actualizar
```python
# 1. Buscar (devuelve Pedido de dominio)
pedido = repo.obtener_por_numero_tracking("TRK-001")

# 2. Usar lógica de negocio
if pedido.is_pendiente():
    pedido.asignar_transportista(uuid4())

# 3. Guardar cambios
repo.guardar(pedido)
```

### Ejemplo 3: Queries complejas
```python
# Obtener pedidos pendientes
pendientes = repo.obtener_pendientes()

# Obtener por cliente
pedidos_cliente = repo.obtener_por_cliente(cliente_id)

# Obtener por transportista
pedidos_transportista = repo.obtener_por_transportista(transportista_id)
```

---

## 🚀 Próximos Pasos

### Para otras entidades (Cliente, Transportista, etc.):

1. **Crear modelo DB**
   ```python
   # infrastructure/persistence/models/cliente_db.py
   class ClienteDB(Base):
       __tablename__ = "clientes"
       # ...
   ```

2. **Crear mapper**
   ```python
   # infrastructure/persistence/mappers/cliente_mapper.py
   class ClienteMapper:
       # ...
   ```

3. **Actualizar repositorio**
   ```python
   # infrastructure/persistence/cliente_repository_sqlalchemy.py
   class ClienteRepositorySQLAlchemy(IClienteRepository):
       def __init__(self, session: Session):
           self.mapper = ClienteMapper()
       # ...
   ```

---

## 📚 Archivos de Referencia

1. 📘 **README.md** - Documentación completa
2. 📗 **example_usage.py** - Ejemplos prácticos
3. 📙 **SOLUTION_SUMMARY.md** - Este archivo

---

## 🎓 Conceptos Clave

### Type Hints en Mapper
```python
# ⚠️ Pylance ve Column, pero en runtime es el valor real
id=pedido_db.id,  # type: ignore
```

### Uso de merge()
```python
# merge() maneja INSERT o UPDATE automáticamente
self.session.merge(pedido_db)
```

### Conversión bidireccional
```python
# Dominio → DB
pedido_db = mapper.to_persistence(pedido)

# DB → Dominio
pedido = mapper.to_domain(pedido_db)
```

---

## ✅ Checklist de Implementación

- [x] Crear modelo SQLAlchemy (`PedidoDB`)
- [x] Crear mapper bidireccional (`PedidoMapper`)
- [x] Actualizar repositorio para usar mapper
- [x] Agregar documentación completa
- [x] Crear ejemplos de uso
- [x] Resolver errores de Pylance
- [x] Mantener separación limpia dominio/infraestructura

---

## 🐛 Troubleshooting

### Error: "Import could not be resolved"
**Causa:** Archivos recién creados, Pylance no los ha indexado.
**Solución:** Reinicia el servidor de lenguaje de VS Code.

### Error: "Column is not assignable to str"
**Causa:** Pylance ve la definición de clase, no el runtime.
**Solución:** Usa `# type: ignore` en las conversiones del mapper.

### Error: "Method overrides class in incompatible manner"
**Causa:** Nombres de parámetros diferentes en interfaz vs implementación.
**Solución:** Ya corregido en el repositorio actualizado.

---

**🎉 Implementación Completa - Lista para Producción**

Creado: 10 de octubre de 2025  
Versión: 1.0  
Estado: ✅ Completo
