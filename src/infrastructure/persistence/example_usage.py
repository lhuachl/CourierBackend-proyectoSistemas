"""
Ejemplo de uso del Patrón de Doble Modelo
==========================================

Este archivo muestra cómo usar el patrón de doble modelo en la práctica.
"""

from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from uuid import uuid4
from datetime import datetime, timedelta
from decimal import Decimal

# Importar entidad de dominio (Pydantic)
from ...models.pedidos import Pedido
from ...models.enums import EstadoPedidoEnum, PrioridadEnum

# Importar implementación de persistencia
from .pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
from .models.pedido_db import Base

def ejemplo_basico():
    """Ejemplo básico de uso del repositorio"""
    
    # 1. Configurar SQLAlchemy
    engine = create_engine('postgresql://user:pass@localhost/courier_db')
    Base.metadata.create_all(engine)
    Session = sessionmaker(bind=engine)
    session = Session()
    
    # 2. Crear repositorio
    repo = PedidoRepositorySQLAlchemy(session)
    
    # 3. Crear entidad de dominio (Pydantic)
    pedido = Pedido(
        numero_tracking="TRK-2025-001",
        id_cliente=uuid4(),
        fecha_solicitud=datetime.utcnow(),
        fecha_entrega_estimada=datetime.utcnow() + timedelta(days=3),
        direccion_origen=uuid4(),
        direccion_destino=uuid4(),
        estado=EstadoPedidoEnum.pendiente,
        prioridad=PrioridadEnum.express,
        peso=Decimal("5.50"),
        dimensiones="30x20x10",
        monto_total=Decimal("150.00")
    )
    
    # 4. Guardar (el mapper convierte automáticamente)
    print(f"Guardando pedido {pedido.numero_tracking}...")
    repo.guardar(pedido)
    print("✅ Pedido guardado exitosamente")
    
    # 5. Obtener por ID (devuelve entidad de dominio)
    print(f"\nBuscando pedido por ID {pedido.id}...")
    pedido_encontrado = repo.obtener_por_id(pedido.id)
    
    if pedido_encontrado:
        print(f"✅ Pedido encontrado: {pedido_encontrado.numero_tracking}")
        print(f"   Estado: {pedido_encontrado.estado}")
        print(f"   Prioridad: {pedido_encontrado.prioridad}")
    
    # 6. Usar lógica de negocio de la entidad
    print("\nAsignando transportista...")
    transportista_id = uuid4()
    pedido_encontrado.asignar_transportista(transportista_id)
    repo.guardar(pedido_encontrado)
    print(f"✅ Transportista {transportista_id} asignado")
    
    # 7. Buscar por tracking
    print(f"\nBuscando por número de tracking...")
    pedido_por_tracking = repo.obtener_por_numero_tracking("TRK-2025-001")
    if pedido_por_tracking:
        print(f"✅ Encontrado: {pedido_por_tracking.estado}")
    
    # 8. Obtener pedidos pendientes
    print("\nObteniendo pedidos pendientes...")
    pedidos_pendientes = repo.obtener_pendientes()
    print(f"✅ Pedidos pendientes: {len(pedidos_pendientes)}")
    
    # Cerrar sesión
    session.close()

def ejemplo_operaciones_crud():
    """Ejemplo de operaciones CRUD completas"""
    
    engine = create_engine('postgresql://user:pass@localhost/courier_db')
    Session = sessionmaker(bind=engine)
    session = Session()
    repo = PedidoRepositorySQLAlchemy(session)
    
    # CREATE
    pedido = Pedido(
        numero_tracking="TRK-CRUD-001",
        id_cliente=uuid4(),
        fecha_solicitud=datetime.utcnow(),
        fecha_entrega_estimada=datetime.utcnow() + timedelta(days=2),
        direccion_origen=uuid4(),
        direccion_destino=uuid4(),
        estado=EstadoPedidoEnum.pendiente,
        prioridad=PrioridadEnum.normal,
        peso=Decimal("3.20"),
        monto_total=Decimal("80.00")
    )
    repo.guardar(pedido)
    print(f"✅ CREATE: Pedido {pedido.numero_tracking} creado")
    
    # READ
    pedido_leido = repo.obtener_por_id(pedido.id)
    print(f"✅ READ: Estado actual = {pedido_leido.estado}")
    
    # UPDATE
    pedido_leido.asignar_transportista(uuid4())
    repo.guardar(pedido_leido)
    print(f"✅ UPDATE: Estado cambiado a {pedido_leido.estado}")
    
    # DELETE
    repo.eliminar(pedido.id)
    print(f"✅ DELETE: Pedido eliminado")
    
    session.close()

def ejemplo_queries_complejas():
    """Ejemplo de queries más complejas"""
    
    engine = create_engine('postgresql://user:pass@localhost/courier_db')
    Session = sessionmaker(bind=engine)
    session = Session()
    repo = PedidoRepositorySQLAlchemy(session)
    
    cliente_id = uuid4()
    transportista_id = uuid4()
    
    # Crear varios pedidos para el mismo cliente
    for i in range(3):
        pedido = Pedido(
            numero_tracking=f"TRK-CLIENTE-{i:03d}",
            id_cliente=cliente_id,
            fecha_solicitud=datetime.utcnow(),
            fecha_entrega_estimada=datetime.utcnow() + timedelta(days=2),
            direccion_origen=uuid4(),
            direccion_destino=uuid4(),
            estado=EstadoPedidoEnum.pendiente if i == 0 else EstadoPedidoEnum.en_ruta,
            prioridad=PrioridadEnum.normal,
            peso=Decimal("2.50"),
            monto_total=Decimal("50.00"),
            id_transportista=transportista_id if i > 0 else None
        )
        repo.guardar(pedido)
    
    # Obtener todos los pedidos del cliente
    pedidos_cliente = repo.obtener_por_cliente(cliente_id)
    print(f"✅ Pedidos del cliente: {len(pedidos_cliente)}")
    
    # Obtener pedidos del transportista
    pedidos_transportista = repo.obtener_por_transportista(transportista_id)
    print(f"✅ Pedidos del transportista: {len(pedidos_transportista)}")
    
    # Obtener pedidos en ruta
    pedidos_en_ruta = repo.obtener_en_ruta()
    print(f"✅ Pedidos en ruta: {len(pedidos_en_ruta)}")
    
    session.close()

def ejemplo_con_logica_negocio():
    """Ejemplo usando métodos de lógica de negocio"""
    
    engine = create_engine('postgresql://user:pass@localhost/courier_db')
    Session = sessionmaker(bind=engine)
    session = Session()
    repo = PedidoRepositorySQLAlchemy(session)
    
    # Crear pedido
    pedido = Pedido(
        numero_tracking="TRK-BUSINESS-001",
        id_cliente=uuid4(),
        fecha_solicitud=datetime.utcnow(),
        fecha_entrega_estimada=datetime.utcnow() + timedelta(days=1),
        direccion_origen=uuid4(),
        direccion_destino=uuid4(),
        estado=EstadoPedidoEnum.pendiente,
        prioridad=PrioridadEnum.express,
        peso=Decimal("1.50"),
        monto_total=Decimal("120.00")
    )
    repo.guardar(pedido)
    
    # Verificar si es express
    if pedido.is_express():
        print("⚡ Pedido express detectado - priorizar")
    
    # Verificar si está pendiente
    if pedido.is_pendiente():
        print("⏳ Pedido pendiente - asignar transportista")
        pedido.asignar_transportista(uuid4())
        repo.guardar(pedido)
    
    # Marcar como entregado
    pedido_actualizado = repo.obtener_por_id(pedido.id)
    pedido_actualizado.marcar_entregado()
    repo.guardar(pedido_actualizado)
    
    if pedido_actualizado.is_entregado():
        print("✅ Pedido entregado exitosamente")
        print(f"   Días desde solicitud: {pedido_actualizado.dias_desde_solicitud}")
    
    session.close()

if __name__ == "__main__":
    print("=" * 60)
    print("EJEMPLO 1: Uso Básico")
    print("=" * 60)
    # ejemplo_basico()
    
    print("\n" + "=" * 60)
    print("EJEMPLO 2: Operaciones CRUD")
    print("=" * 60)
    # ejemplo_operaciones_crud()
    
    print("\n" + "=" * 60)
    print("EJEMPLO 3: Queries Complejas")
    print("=" * 60)
    # ejemplo_queries_complejas()
    
    print("\n" + "=" * 60)
    print("EJEMPLO 4: Lógica de Negocio")
    print("=" * 60)
    # ejemplo_con_logica_negocio()
    
    print("\n⚠️  Descomenta las funciones para ejecutar los ejemplos")
    print("⚠️  Asegúrate de configurar la conexión a la base de datos")
