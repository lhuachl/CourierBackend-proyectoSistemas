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
   # Editar .env con tus valores
   ```
   
   Ver **[ENV.md](ENV.md)** para documentación completa de configuración.

3. **Iniciar el servidor:**
   ```bash
   python main.py
   ```

4. **Acceder a la documentación:**
   - Swagger UI: http://localhost:8000/docs
   - ReDoc: http://localhost:8000/redoc

## 📚 Documentación

- **[API_USAGE.md](API_USAGE.md)** - Guía completa de uso de la API
- **[ENV.md](ENV.md)** - Configuración de variables de entorno y Supabase
- **[daily-scrum2.md](daily-scrum2.md)** - Detalles de implementación

## 🔧 Tecnologías

- FastAPI
- SQLAlchemy
- Pydantic
- JWT (python-jose)
- Supabase (PostgreSQL)

## 📝 Configuración con Supabase

Este proyecto está configurado para usar Supabase (PostgreSQL) en producción. Ver [ENV.md](ENV.md) para:
- Instrucciones detalladas de configuración
- Esquema de base de datos
- Configuración del plan gratuito
- Troubleshooting

## 🔐 Seguridad

- Autenticación JWT
- Hash de contraseñas con bcrypt
- Control de acceso basado en roles (RBAC)
- Variables de entorno para secretos
