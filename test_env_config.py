#!/usr/bin/env python3
"""
Script de prueba para verificar la configuración de variables de entorno
"""
import os
import sys
from dotenv import load_dotenv

def test_env_loading():
    """Prueba la carga de variables de entorno"""
    print("=" * 60)
    print("PRUEBA DE CONFIGURACIÓN DE VARIABLES DE ENTORNO")
    print("=" * 60)
    
    # Cargar .env si existe
    env_file_exists = os.path.exists('.env')
    if env_file_exists:
        print("✅ Archivo .env encontrado")
        load_dotenv()
    else:
        print("⚠️  Archivo .env no encontrado - usando valores por defecto")
    
    print("\n--- Variables de Base de Datos ---")
    database_url = os.getenv('DATABASE_URL', 'sqlite:///./courier.db')
    print(f"DATABASE_URL: {database_url}")
    
    if database_url.startswith('postgresql'):
        print("  → Tipo: PostgreSQL/Supabase")
        print("  ⚠️  Asegúrate de tener instalado: pip install psycopg2-binary")
    elif database_url.startswith('sqlite'):
        print("  → Tipo: SQLite (desarrollo)")
    
    print("\n--- Variables de Seguridad JWT ---")
    secret_key = os.getenv('SECRET_KEY', 'default-secret-key')
    algorithm = os.getenv('JWT_ALGORITHM', 'HS256')
    expire_minutes = os.getenv('ACCESS_TOKEN_EXPIRE_MINUTES', '30')
    
    print(f"SECRET_KEY: {secret_key[:20]}... (primeros 20 caracteres)")
    print(f"JWT_ALGORITHM: {algorithm}")
    print(f"ACCESS_TOKEN_EXPIRE_MINUTES: {expire_minutes}")
    
    # Validaciones
    print("\n--- Validaciones ---")
    warnings = []
    
    if secret_key == 'tu-clave-secreta-super-segura-cambiar-en-produccion':
        warnings.append("⚠️  Usando SECRET_KEY por defecto - cambiar en producción")
    
    if not env_file_exists:
        warnings.append("⚠️  No se encontró archivo .env - copiar .env.example a .env")
    
    if warnings:
        for warning in warnings:
            print(warning)
    else:
        print("✅ Todas las validaciones pasaron")
    
    print("\n" + "=" * 60)
    print("PRUEBA COMPLETADA")
    print("=" * 60)
    
    # Intentar importar los módulos
    print("\n--- Prueba de Importación de Módulos ---")
    try:
        from infrastructure.auth.security import SECRET_KEY as imported_key
        print(f"✅ infrastructure.auth.security importado correctamente")
        print(f"   SECRET_KEY: {imported_key[:20]}...")
    except Exception as e:
        print(f"❌ Error al importar infrastructure.auth.security: {e}")
        return False
    
    try:
        # Solo importar DATABASE_URL, no el engine si es PostgreSQL sin psycopg2
        if not database_url.startswith('postgresql'):
            from api.dependencies import DATABASE_URL as imported_db_url
            print(f"✅ api.dependencies importado correctamente")
            print(f"   DATABASE_URL: {imported_db_url}")
        else:
            print("⚠️  Saltando importación de api.dependencies (PostgreSQL requiere psycopg2)")
    except Exception as e:
        print(f"❌ Error al importar api.dependencies: {e}")
        return False
    
    print("\n✅ Todas las pruebas pasaron correctamente")
    return True

if __name__ == "__main__":
    success = test_env_loading()
    sys.exit(0 if success else 1)
