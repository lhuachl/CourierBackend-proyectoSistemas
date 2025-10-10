# Patrón de Doble Modelo - Infrastructure Layer

## 📋 Descripción

Esta capa implementa el **patrón de doble modelo** para separar completamente la lógica de dominio de la persistencia. Este patrón permite que las entidades de dominio (Pydantic) permanezcan agnósticas a la tecnología de base de datos, mientras que los modelos de persistencia (SQLAlchemy) manejan la interacción con la base de datos.

## 🏗️ Estructura

```
infrastructure/
└── persistence/
    ├── models/              # Modelos SQLAlchemy (capa de persistencia)
    │   ├── __init__.py
    │   └── pedido_db.py    # Modelo PedidoDB
    │
    ├── mappers/             # Conversores entre dominio y persistencia
    │   ├── __init__.py
    │   └── pedido_mapper.py # PedidoMapper
    │
    └── pedido_repository_sqlalchemy.py  # Implementación del repositorio
```

## 🔄 Flujo de Datos

```
┌─────────────────────┐
│  Capa de Aplicación │
│   (FastAPI, etc.)   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Entidad de Dominio │
│   Pedido (Pydantic) │  ← Agnóstico a tecnología
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   PedidoMapper      │  ← Convierte entre modelos
│  (to_domain/        │
│   to_persistence)   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Modelo Persistencia│
│  PedidoDB (SQLAlchemy)│ ← Interactúa con BD
└─────────────────────┘
```

## 📦 Componentes

### 1. Modelo de Dominio (Pydantic)
**Archivo:** `models/pedidos/pedido.py`

```python
class Pedido(TimestampedEntity):
    """Entidad Pedido del dominio - Agnóstica a tecnología"""
    numero_tracking: str
    id_cliente: UUID
    estado: EstadoPedidoEnum
    # ... otros campos
    
    # Lógica de negocio
    def asignar_transportista(self, transportista_id: UUID):
        # ...
```

**Características:**
- ✅ Validación con Pydantic
- ✅ Lógica de negocio pura
- ✅ Sin dependencias de SQLAlchemy
- ✅ Fácil de testear

### 2. Modelo de Persistencia (SQLAlchemy)
**Archivo:** `infrastructure/persistence/models/pedido_db.py`

```python
class PedidoDB(Base):
    """Modelo SQLAlchemy para persistencia de pedidos"""
    __tablename__ = "pedidos"
    
    id = Column(PGUUID(as_uuid=True), primary_key=True)
    numero_tracking = Column(String, unique=True, nullable=False, index=True)
    estado = Column(SQLEnum(EstadoPedidoEnum), nullable=False, index=True)
    # ... otros campos con metadatos de BD
```

**Características:**
- ✅ Mapeo ORM con SQLAlchemy
- ✅ Índices y constraints de BD
- ✅ Relaciones entre tablas
- ✅ Sin lógica de negocio

### 3. Mapper (Convertidor)
**Archivo:** `infrastructure/persistence/mappers/pedido_mapper.py`

```python
class PedidoMapper:
    @staticmethod
    def to_domain(pedido_db: PedidoDB) -> Pedido:
        """Convierte SQLAlchemy → Pydantic"""
        return Pedido(
            id=pedido_db.id,  # type: ignore
            numero_tracking=pedido_db.numero_tracking,  # type: ignore
            # ...
        )
    
    @staticmethod
    def to_persistence(pedido: Pedido) -> PedidoDB:
        """Convierte Pydantic → SQLAlchemy"""
        return PedidoDB(
            id=pedido.id,
            numero_tracking=pedido.numero_tracking,
            # ...
        )
```

**Nota sobre `# type: ignore`:**
Los comentarios `# type: ignore` son necesarios porque:
- En **definición de clase**: Los atributos son `Column` (metadatos)
- En **tiempo de ejecución**: SQLAlchemy los convierte a valores reales
- Pylance ve la definición, no el comportamiento en runtime

### 4. Repositorio (Implementación)
**Archivo:** `infrastructure/persistence/pedido_repository_sqlalchemy.py`

```python
class PedidoRepositorySQLAlchemy(IPedidoRepository):
    def __init__(self, session: Session):
        self.session = session
        self.mapper = PedidoMapper()
    
    def guardar(self, pedido: Pedido) -> None:
        # 1. Convertir dominio → persistencia
        pedido_db = self.mapper.to_persistence(pedido)
        
        # 2. Guardar con SQLAlchemy
        self.session.merge(pedido_db)
        self.session.commit()
    
    def obtener_por_id(self, id: UUID) -> Optional[Pedido]:
        # 1. Query con SQLAlchemy (SÍN errores de Pylance ✅)
        pedido_db = self.session.query(PedidoDB).filter(
            PedidoDB.id == id
        ).first()
        
        # 2. Convertir persistencia → dominio
        return self.mapper.to_domain(pedido_db) if pedido_db else None
```

## ✅ Ventajas del Patrón

1. **Separación de Responsabilidades**
   - Dominio: Lógica de negocio
   - Persistencia: Almacenamiento

2. **Sin Errores de Pylance**
   - Los queries usan `PedidoDB` (SQLAlchemy)
   - Las entidades usan `Pedido` (Pydantic)

3. **Testeable**
   - Puedes testear `Pedido` sin BD
   - Puedes mockear el mapper

4. **Flexibilidad**
   - Cambiar de BD sin tocar el dominio
   - Agregar validaciones Pydantic fácilmente

5. **Clean Architecture**
   - El dominio no conoce la infraestructura
   - La infraestructura depende del dominio

## 🔧 Uso en la Aplicación

```python
from sqlalchemy.orm import Session
from infrastructure.persistence.pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
from models.pedidos import Pedido
from uuid import uuid4

# 1. Crear repositorio con sesión de SQLAlchemy
session = Session(engine)
repo = PedidoRepositorySQLAlchemy(session)

# 2. Trabajar con entidades de dominio (Pydantic)
pedido = Pedido(
    numero_tracking="TRK-001",
    id_cliente=uuid4(),
    # ...
)

# 3. Guardar (el mapper convierte automáticamente)
repo.guardar(pedido)

# 4. Obtener (devuelve entidad de dominio)
pedido_encontrado = repo.obtener_por_id(pedido.id)

# 5. Usar lógica de negocio
pedido_encontrado.asignar_transportista(uuid4())
repo.guardar(pedido_encontrado)
```

## 📚 Extender el Patrón

Para agregar más entidades (Cliente, Transportista, etc.):

1. **Crear modelo de persistencia:**
   ```python
   # infrastructure/persistence/models/cliente_db.py
   class ClienteDB(Base):
       __tablename__ = "clientes"
       # ...
   ```

2. **Crear mapper:**
   ```python
   # infrastructure/persistence/mappers/cliente_mapper.py
   class ClienteMapper:
       @staticmethod
       def to_domain(cliente_db: ClienteDB) -> Cliente:
           # ...
   ```

3. **Actualizar repositorio:**
   ```python
   # infrastructure/persistence/cliente_repository_sqlalchemy.py
   class ClienteRepositorySQLAlchemy(IClienteRepository):
       def __init__(self, session: Session):
           self.session = session
           self.mapper = ClienteMapper()
       # ...
   ```

## 🐛 Troubleshooting

### Problema: Errores de Pylance en el mapper

```python
# ❌ Error: Cannot assign to attribute
pedido_db.numero_tracking = pedido.numero_tracking
```

**Solución:** Agregar `# type: ignore`
```python
# ✅ Sin error
pedido_db.numero_tracking = pedido.numero_tracking  # type: ignore
```

### Problema: Query con Pydantic

```python
# ❌ Error: Pedido no tiene columnas SQLAlchemy
self.session.query(Pedido).filter(Pedido.estado == "pendiente")
```

**Solución:** Usar modelo de persistencia
```python
# ✅ Correcto
self.session.query(PedidoDB).filter(PedidoDB.estado == "pendiente")
```

## 📖 Referencias

- [Clean Architecture - Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Pydantic Documentation](https://docs.pydantic.dev/)
- [SQLAlchemy ORM](https://docs.sqlalchemy.org/en/20/orm/)
- [Repository Pattern](https://martinfowler.com/eaaCatalog/repository.html)

---

**Creado:** 10 de octubre de 2025  
**Autor:** Sistema CourierBackend  
**Versión:** 1.0
