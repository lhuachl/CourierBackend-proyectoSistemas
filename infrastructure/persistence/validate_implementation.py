"""
Script de validación de la implementación del Patrón de Doble Modelo
=====================================================================

Este script valida que la implementación esté correcta y sin errores.
"""

import sys
from pathlib import Path

def validate_structure():
    """Valida que todos los archivos necesarios existan"""
    print("🔍 Validando estructura de archivos...")
    
    base_path = Path(__file__).parent
    required_files = [
        "models/__init__.py",
        "models/pedido_db.py",
        "mappers/__init__.py",
        "mappers/pedido_mapper.py",
        "pedido_repository_sqlalchemy.py",
        "README.md",
        "SOLUTION_SUMMARY.md",
        "example_usage.py",
    ]
    
    all_exist = True
    for file_path in required_files:
        full_path = base_path / file_path
        if full_path.exists():
            print(f"  ✅ {file_path}")
        else:
            print(f"  ❌ {file_path} - NO EXISTE")
            all_exist = False
    
    return all_exist

def validate_imports():
    """Valida que los imports funcionen correctamente"""
    print("\n🔍 Validando imports...")
    
    try:
        from .models import PedidoDB
        print("  ✅ PedidoDB importado correctamente")
    except Exception as e:
        print(f"  ❌ Error importando PedidoDB: {e}")
        return False
    
    try:
        from .mappers import PedidoMapper
        print("  ✅ PedidoMapper importado correctamente")
    except Exception as e:
        print(f"  ❌ Error importando PedidoMapper: {e}")
        return False
    
    try:
        from .pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
        print("  ✅ PedidoRepositorySQLAlchemy importado correctamente")
    except Exception as e:
        print(f"  ❌ Error importando PedidoRepositorySQLAlchemy: {e}")
        return False
    
    return True

def validate_pattern_implementation():
    """Valida que el patrón esté implementado correctamente"""
    print("\n🔍 Validando implementación del patrón...")
    
    try:
        from .models.pedido_db import PedidoDB
        from .mappers.pedido_mapper import PedidoMapper
        from sqlalchemy import inspect
        
        # Validar que PedidoDB tenga las columnas correctas
        mapper = inspect(PedidoDB)
        columns = [col.name for col in mapper.columns]
        
        required_columns = [
            'id', 'numero_tracking', 'id_cliente', 'fecha_solicitud',
            'fecha_entrega_estimada', 'estado', 'prioridad', 'peso',
            'monto_total', 'created_at', 'updated_at'
        ]
        
        missing_columns = [col for col in required_columns if col not in columns]
        if missing_columns:
            print(f"  ❌ Columnas faltantes en PedidoDB: {missing_columns}")
            return False
        
        print(f"  ✅ PedidoDB tiene todas las columnas requeridas ({len(columns)} columnas)")
        
        # Validar que PedidoMapper tenga los métodos correctos
        if not hasattr(PedidoMapper, 'to_domain'):
            print("  ❌ PedidoMapper no tiene método to_domain")
            return False
        
        if not hasattr(PedidoMapper, 'to_persistence'):
            print("  ❌ PedidoMapper no tiene método to_persistence")
            return False
        
        print("  ✅ PedidoMapper tiene todos los métodos requeridos")
        
        return True
        
    except Exception as e:
        print(f"  ❌ Error validando implementación: {e}")
        return False

def validate_repository_methods():
    """Valida que el repositorio tenga todos los métodos requeridos"""
    print("\n🔍 Validando métodos del repositorio...")
    
    try:
        from .pedido_repository_sqlalchemy import PedidoRepositorySQLAlchemy
        
        required_methods = [
            'guardar',
            'obtener_por_id',
            'eliminar',
            'obtener_por_numero_tracking',
            'obtener_por_cliente',
            'obtener_por_transportista',
            'obtener_pendientes',
            'obtener_en_ruta',
        ]
        
        missing_methods = [
            method for method in required_methods 
            if not hasattr(PedidoRepositorySQLAlchemy, method)
        ]
        
        if missing_methods:
            print(f"  ❌ Métodos faltantes: {missing_methods}")
            return False
        
        print(f"  ✅ PedidoRepositorySQLAlchemy tiene todos los métodos requeridos ({len(required_methods)} métodos)")
        
        # Validar que use el mapper
        repo_code = inspect.getsource(PedidoRepositorySQLAlchemy)
        if 'self.mapper' not in repo_code:
            print("  ❌ El repositorio no usa el mapper")
            return False
        
        print("  ✅ El repositorio usa el mapper correctamente")
        
        return True
        
    except Exception as e:
        print(f"  ❌ Error validando repositorio: {e}")
        return False

def main():
    """Función principal de validación"""
    print("=" * 70)
    print("VALIDACIÓN DE PATRÓN DE DOBLE MODELO")
    print("=" * 70)
    
    results = []
    
    # Ejecutar validaciones
    results.append(("Estructura de archivos", validate_structure()))
    results.append(("Imports", validate_imports()))
    results.append(("Implementación del patrón", validate_pattern_implementation()))
    results.append(("Métodos del repositorio", validate_repository_methods()))
    
    # Mostrar resumen
    print("\n" + "=" * 70)
    print("RESUMEN DE VALIDACIÓN")
    print("=" * 70)
    
    all_passed = True
    for name, passed in results:
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"{status} - {name}")
        if not passed:
            all_passed = False
    
    print("=" * 70)
    
    if all_passed:
        print("\n🎉 ¡Todas las validaciones pasaron exitosamente!")
        print("✅ El patrón de doble modelo está correctamente implementado")
        return 0
    else:
        print("\n⚠️  Algunas validaciones fallaron")
        print("❌ Revisa los errores anteriores")
        return 1

if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as e:
        print(f"\n💥 Error fatal durante la validación: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
