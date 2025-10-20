# Sistema Courier - Backend

API RESTful para sistema de gestión de pedidos y entregas de courier.

## 🚀 Inicio Rápido

1. **Instalar dependencias:**
   ```bash
   pip install fastapi sqlalchemy pydantic "pydantic[email]" passlib python-jose python-multipart uvicorn python-dotenv
   ```

2. **Configurar variables de entorno:**
   ```bash
   cp .env.example .env
   # Editar .env con tus valores de Supabase Session Pooler
   ```
   
   Ver **[ENV.md](ENV.md)** para documentación completa de configuración.
   Ver **[SUPABASE_POOLER_SETUP.md](SUPABASE_POOLER_SETUP.md)** para guía de configuración del Session Pooler.

3. **Probar la conexión (opcional):**
   ```bash
   python test_supabase_connection.py
   ```

4. **Iniciar el servidor:**
   ```bash
   python main.py
   ```

5. **Acceder a la documentación:**
   - Swagger UI: http://localhost:8000/docs
   - ReDoc: http://localhost:8000/redoc

## 📚 Documentación

- **[API_USAGE.md](API_USAGE.md)** - Guía completa de uso de la API
- **[ENV.md](ENV.md)** - Configuración de variables de entorno y Supabase
- **[SUPABASE_POOLER_SETUP.md](SUPABASE_POOLER_SETUP.md)** - Guía de configuración del Session Pooler
- **[daily-scrum2.md](daily-scrum2.md)** - Detalles de implementación

## 🔧 Tecnologías

- FastAPI
- SQLAlchemy
- Pydantic
- JWT (python-jose)
- Supabase (PostgreSQL)

## 📝 Configuración con Supabase

Este proyecto está configurado para usar **Supabase Session Pooler** (PostgreSQL) en producción. 

### Configuración Recomendada: Session Pooler

El proyecto ahora soporta la configuración recomendada por Supabase usando parámetros individuales:

```bash
user=postgres.hlmngthhnvbdvbrxukqy
password=[YOUR_PASSWORD]
host=aws-1-us-east-2.pooler.supabase.com
port=5432
dbname=postgres
```

**Beneficios:**
- ✅ Mejor rendimiento con SQLAlchemy
- ✅ Gestión automática de conexiones
- ✅ Configuración optimizada con `NullPool`
- ✅ SSL habilitado automáticamente

Ver [SUPABASE_POOLER_SETUP.md](SUPABASE_POOLER_SETUP.md) y [ENV.md](ENV.md) para:
- Instrucciones detalladas de configuración
- Esquema de base de datos
- Configuración del plan gratuito
- Troubleshooting

## 🔐 Seguridad

- Autenticación JWT
- Hash de contraseñas con bcrypt
- Control de acceso basado en roles (RBAC)
- Variables de entorno para secretos
