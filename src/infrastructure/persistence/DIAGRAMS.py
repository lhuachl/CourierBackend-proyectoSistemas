"""
Diagrama Visual del Patrón de Doble Modelo
===========================================

Este archivo contiene diagramas ASCII para entender el patrón.
"""

ARCHITECTURE_DIAGRAM = """
╔══════════════════════════════════════════════════════════════════════╗
║                    ARQUITECTURA DE CAPAS                              ║
╚══════════════════════════════════════════════════════════════════════╝

┌──────────────────────────────────────────────────────────────────────┐
│                         APLICACIÓN                                    │
│                    (FastAPI, Controllers)                             │
│                                                                       │
│  • Endpoints REST                                                     │
│  • Validación de requests                                             │
│  • Serialización de responses                                         │
└───────────────────────────┬──────────────────────────────────────────┘
                            │
                            │ usa
                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│                    CAPA DE DOMINIO                                    │
│                  (Entidades + Repositorios)                           │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │              Pedido (Pydantic Model)                         │    │
│  │                                                              │    │
│  │  • Atributos de negocio                                      │    │
│  │  • Validaciones Pydantic                                     │    │
│  │  • Métodos de negocio:                                       │    │
│  │    - asignar_transportista()                                 │    │
│  │    - marcar_entregado()                                      │    │
│  │    - cancelar()                                              │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │        IPedidoRepository (Interface)                         │    │
│  │                                                              │    │
│  │  • guardar(entidad)                                          │    │
│  │  • obtener_por_id(id)                                        │    │
│  │  • obtener_pendientes()                                      │    │
│  │  • ...                                                        │    │
│  └─────────────────────────────────────────────────────────────┘    │
└───────────────────────────┬──────────────────────────────────────────┘
                            │
                            │ implementa
                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│                  CAPA DE INFRAESTRUCTURA                              │
│                      (Persistencia)                                   │
│                                                                       │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │   PedidoRepositorySQLAlchemy                                 │    │
│  │                                                              │    │
│  │   def obtener_por_id(id):                                    │    │
│  │       1. pedido_db = query(PedidoDB)...    ← SQLAlchemy     │    │
│  │       2. return mapper.to_domain(pedido_db) ← Pydantic      │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                            │                                          │
│                            │ usa                                      │
│                            ▼                                          │
│  ┌──────────────────────────────────────────────────┐                │
│  │           PedidoMapper                            │                │
│  │                                                   │                │
│  │  to_domain():      PedidoDB  →  Pedido           │                │
│  │  to_persistence(): Pedido    →  PedidoDB         │                │
│  └──────────────────────────────────────────────────┘                │
│                            │                                          │
│                            │ convierte                                │
│                            ▼                                          │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │              PedidoDB (SQLAlchemy Model)                     │    │
│  │                                                              │    │
│  │  • Columnas SQLAlchemy                                       │    │
│  │  • Índices de BD                                             │    │
│  │  • Constraints                                               │    │
│  │  • Sin lógica de negocio                                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
└───────────────────────────┬──────────────────────────────────────────┘
                            │
                            │ persiste
                            ▼
┌──────────────────────────────────────────────────────────────────────┐
│                      BASE DE DATOS                                    │
│                   (PostgreSQL / MySQL)                                │
│                                                                       │
│  Tabla: pedidos                                                       │
│  ├─ id (UUID, PK)                                                     │
│  ├─ numero_tracking (STRING, UNIQUE, INDEX)                           │
│  ├─ estado (ENUM, INDEX)                                              │
│  └─ ...                                                               │
└──────────────────────────────────────────────────────────────────────┘
"""

DATA_FLOW_DIAGRAM = """
╔══════════════════════════════════════════════════════════════════════╗
║                     FLUJO DE DATOS - LECTURA                          ║
╚══════════════════════════════════════════════════════════════════════╝

GET /pedidos/{id}
       │
       ▼
┌────────────────┐
│  Controller    │  1. Recibe request
└────────┬───────┘
         │
         ▼
┌────────────────────────────┐
│  PedidoRepository          │  2. Llama a obtener_por_id(id)
│  .obtener_por_id(id)       │
└────────┬───────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  SQLAlchemy Query                         │  3. Query con PedidoDB
│  session.query(PedidoDB)                  │     (NO con Pedido ✅)
│    .filter(PedidoDB.id == id)             │
│    .first()                               │
└────────┬─────────────────────────────────┘
         │
         │ Resultado: pedido_db
         ▼
┌──────────────────────────────────────────┐
│  PedidoDB                                 │  4. Instancia de SQLAlchemy
│  (SQLAlchemy Model Instance)              │
│                                           │
│  id = UUID(...)                           │
│  numero_tracking = "TRK-001"              │
│  estado = EstadoPedidoEnum.pendiente      │
└────────┬─────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  PedidoMapper.to_domain(pedido_db)        │  5. Convierte a dominio
└────────┬─────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  Pedido                                   │  6. Entidad de dominio
│  (Pydantic Model Instance)                │
│                                           │
│  id = UUID(...)                           │
│  numero_tracking = "TRK-001"              │
│  estado = EstadoPedidoEnum.pendiente      │
│                                           │
│  def asignar_transportista() ...          │
│  def marcar_entregado() ...               │
└────────┬─────────────────────────────────┘
         │
         ▼
┌────────────────┐
│  Controller    │  7. Retorna JSON response
└────────────────┘
         │
         ▼
    {"id": "...", "numero_tracking": "TRK-001", ...}


╔══════════════════════════════════════════════════════════════════════╗
║                    FLUJO DE DATOS - ESCRITURA                         ║
╚══════════════════════════════════════════════════════════════════════╝

POST /pedidos
       │
       ▼
┌────────────────┐
│  Controller    │  1. Recibe request + valida
└────────┬───────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  Pedido (Pydantic)                        │  2. Crea entidad de dominio
│                                           │     con validación Pydantic
│  pedido = Pedido(**request_data)          │
└────────┬─────────────────────────────────┘
         │
         ▼
┌────────────────────────────┐
│  PedidoRepository          │  3. Llama a guardar(pedido)
│  .guardar(pedido)          │
└────────┬───────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  PedidoMapper.to_persistence(pedido)      │  4. Convierte a modelo DB
└────────┬─────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  PedidoDB                                 │  5. Instancia de SQLAlchemy
│  (SQLAlchemy Model Instance)              │
│                                           │
│  id = UUID(...)                           │
│  numero_tracking = "TRK-001"              │
│  estado = EstadoPedidoEnum.pendiente      │
└────────┬─────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  SQLAlchemy                               │  6. Persiste en BD
│  session.merge(pedido_db)                 │
│  session.commit()                         │
└────────┬─────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────┐
│  Base de Datos                            │  7. INSERT/UPDATE
│  INSERT INTO pedidos VALUES (...)         │
└────────┬─────────────────────────────────┘
         │
         ▼
┌────────────────┐
│  Controller    │  8. Retorna 201 Created
└────────────────┘
"""

COMPARISON_DIAGRAM = """
╔══════════════════════════════════════════════════════════════════════╗
║                 ANTES vs DESPUÉS DEL PATRÓN                           ║
╚══════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────┐
│                              ANTES ❌                                 │
└─────────────────────────────────────────────────────────────────────┘

class Pedido(TimestampedEntity):  ← Modelo Pydantic
    numero_tracking: str
    estado: EstadoPedidoEnum
    # ...

class PedidoRepository:
    def obtener_pendientes(self):
        # ❌ ERROR: Pedido no tiene columnas SQLAlchemy
        return self.session.query(Pedido).filter(
            Pedido.estado == EstadoPedidoEnum.pendiente
        ).all()
        
Problemas:
❌ Pylance/MyPy errors constantes
❌ Mezcla de lógica de negocio y persistencia
❌ Difícil de testear
❌ Acoplamiento a SQLAlchemy


┌─────────────────────────────────────────────────────────────────────┐
│                              DESPUÉS ✅                               │
└─────────────────────────────────────────────────────────────────────┘

# Entidad de Dominio (Pydantic)
class Pedido(TimestampedEntity):
    numero_tracking: str
    estado: EstadoPedidoEnum
    
    def asignar_transportista(self, id):  # ← Lógica de negocio
        self.id_transportista = id
        self.estado = EstadoPedidoEnum.procesando

# Modelo de Persistencia (SQLAlchemy)
class PedidoDB(Base):
    __tablename__ = "pedidos"
    
    id = Column(PGUUID, primary_key=True)
    numero_tracking = Column(String, unique=True, index=True)
    estado = Column(SQLEnum(EstadoPedidoEnum), index=True)

# Mapper
class PedidoMapper:
    @staticmethod
    def to_domain(pedido_db: PedidoDB) -> Pedido:
        return Pedido(...)
    
    @staticmethod
    def to_persistence(pedido: Pedido) -> PedidoDB:
        return PedidoDB(...)

# Repositorio
class PedidoRepositorySQLAlchemy:
    def __init__(self, session):
        self.session = session
        self.mapper = PedidoMapper()
    
    def obtener_pendientes(self):
        # ✅ SIN ERROR: PedidoDB tiene columnas SQLAlchemy
        pedidos_db = self.session.query(PedidoDB).filter(
            PedidoDB.estado == EstadoPedidoEnum.pendiente
        ).all()
        
        # ✅ Devuelve entidades de dominio
        return [self.mapper.to_domain(p) for p in pedidos_db]

Ventajas:
✅ Sin errores de Pylance/MyPy
✅ Separación clara de responsabilidades
✅ Fácil de testear cada capa
✅ Desacoplado de la tecnología de BD
✅ Sigue Clean Architecture
"""

TESTING_DIAGRAM = """
╔══════════════════════════════════════════════════════════════════════╗
║                         ESTRATEGIA DE TESTING                         ║
╚══════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────┐
│                    1. TEST DE DOMINIO (Unit)                          │
└─────────────────────────────────────────────────────────────────────┘

def test_asignar_transportista():
    # ✅ Sin necesidad de BD
    pedido = Pedido(
        numero_tracking="TRK-001",
        id_cliente=uuid4(),
        estado=EstadoPedidoEnum.pendiente,
        # ...
    )
    
    transportista_id = uuid4()
    pedido.asignar_transportista(transportista_id)
    
    assert pedido.id_transportista == transportista_id
    assert pedido.estado == EstadoPedidoEnum.procesando

┌─────────────────────────────────────────────────────────────────────┐
│                    2. TEST DE MAPPER (Unit)                           │
└─────────────────────────────────────────────────────────────────────┘

def test_mapper_to_domain():
    # Mock de PedidoDB
    pedido_db = Mock(spec=PedidoDB)
    pedido_db.id = uuid4()
    pedido_db.numero_tracking = "TRK-001"
    # ...
    
    # Convertir
    pedido = PedidoMapper.to_domain(pedido_db)
    
    assert isinstance(pedido, Pedido)
    assert pedido.numero_tracking == "TRK-001"

┌─────────────────────────────────────────────────────────────────────┐
│                 3. TEST DE REPOSITORIO (Integration)                  │
└─────────────────────────────────────────────────────────────────────┘

def test_guardar_y_obtener(db_session):
    # Setup
    repo = PedidoRepositorySQLAlchemy(db_session)
    pedido = Pedido(...)
    
    # Guardar
    repo.guardar(pedido)
    
    # Obtener
    pedido_obtenido = repo.obtener_por_id(pedido.id)
    
    assert pedido_obtenido is not None
    assert pedido_obtenido.numero_tracking == pedido.numero_tracking

┌─────────────────────────────────────────────────────────────────────┐
│                     4. TEST END-TO-END (E2E)                          │
└─────────────────────────────────────────────────────────────────────┘

def test_crear_y_actualizar_pedido(client, db_session):
    # Crear pedido via API
    response = client.post("/pedidos", json={
        "numero_tracking": "TRK-001",
        # ...
    })
    assert response.status_code == 201
    
    pedido_id = response.json()["id"]
    
    # Asignar transportista
    response = client.patch(f"/pedidos/{pedido_id}/transportista", json={
        "transportista_id": str(uuid4())
    })
    assert response.status_code == 200
    assert response.json()["estado"] == "procesando"
"""

def print_diagrams():
    """Imprime todos los diagramas"""
    print(ARCHITECTURE_DIAGRAM)
    print("\n" + "=" * 70 + "\n")
    print(DATA_FLOW_DIAGRAM)
    print("\n" + "=" * 70 + "\n")
    print(COMPARISON_DIAGRAM)
    print("\n" + "=" * 70 + "\n")
    print(TESTING_DIAGRAM)

if __name__ == "__main__":
    print_diagrams()
