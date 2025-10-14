# Daily Scrum #2 - Implementación de Login y CRUD Básico

## 📅 Fecha: 2025-10-14

## 🎯 Objetivo
Implementar funcionalidad completa de autenticación (login/registro) y operaciones CRUD básicas para usuarios y pedidos, siguiendo la arquitectura limpia y el patrón de doble modelo ya establecido en el proyecto.

## ✅ Trabajo Completado

### 1. Infraestructura de Autenticación

#### 1.1 Servicios de Seguridad (`infrastructure/auth/`)
- ✅ **security.py**: Implementación de servicios de autenticación
  - Hash de contraseñas usando `passlib` con bcrypt
  - Generación y verificación de tokens JWT usando `python-jose`
  - Funciones implementadas:
    - `get_password_hash()`: Genera hash seguro de contraseña
    - `verify_password()`: Verifica contraseña contra hash
    - `create_access_token()`: Crea token JWT con expiración
    - `decode_access_token()`: Decodifica y valida token JWT

#### 1.2 Modelo de Persistencia para Usuarios
- ✅ **user_db.py**: Modelo SQLAlchemy para tabla `users`
  - Campos: id, email (único), password_hash, nombre, apellido, role, estado
  - Timestamps automáticos (created_at, updated_at)
  - Índice en email para búsquedas rápidas

#### 1.3 Mapper de Usuario
- ✅ **user_mapper.py**: Conversión bidireccional Usuario
  - `to_domain()`: UserDB → User (entidad de dominio Pydantic)
  - `to_persistence()`: User → UserDB (modelo SQLAlchemy)
  - Conversión automática de enums (RoleEnum, EstadoEnum)

#### 1.4 Repositorio de Usuarios
- ✅ **user_repository_sqlalchemy.py**: Implementación de IUserRepository
  - Métodos CRUD básicos:
    - `guardar()`: Crear/actualizar usuario
    - `obtener_por_id()`: Buscar por UUID
    - `eliminar()`: Eliminar usuario
  - Métodos específicos:
    - `obtener_por_email()`: Buscar por email (para login)
    - `obtener_por_rol()`: Filtrar por rol
    - `email_existe()`: Validar email único

### 2. Capa de API REST

#### 2.1 Esquemas Pydantic (`api/schemas/`)
- ✅ **auth_schemas.py**: Esquemas de autenticación
  - `LoginRequest`: Email + password
  - `RegisterRequest`: Datos completos de registro
  - `TokenResponse`: Respuesta con token JWT y datos de usuario

- ✅ **user_schemas.py**: Esquemas de usuario
  - `UserResponse`: Datos públicos del usuario
  - `UserUpdate`: Campos actualizables (nombre, apellido, estado)

- ✅ **pedido_schemas.py**: Esquemas de pedido
  - `PedidoCreate`: Crear nuevo pedido
  - `PedidoUpdate`: Actualizar pedido existente
  - `PedidoResponse`: Respuesta completa de pedido

#### 2.2 Dependencias de FastAPI (`api/dependencies.py`)
- ✅ Configuración de base de datos SQLite (desarrollo)
- ✅ `get_db()`: Inyección de sesión de BD
- ✅ `get_user_repository()`: Inyección de repositorio de usuarios
- ✅ `get_pedido_repository()`: Inyección de repositorio de pedidos
- ✅ `get_current_user()`: Autenticación vía token Bearer
- ✅ `require_admin()`: Middleware para rutas de administrador

#### 2.3 Rutas de Autenticación (`api/routes/auth.py`)
- ✅ **POST /auth/login**
  - Autentica usuario con email y contraseña
  - Retorna token JWT válido por 30 minutos
  - Valida que usuario esté activo

- ✅ **POST /auth/register**
  - Crea nuevo usuario en el sistema
  - Valida que email no esté registrado
  - Hash automático de contraseña
  - Retorna token JWT inmediatamente
  - Soporta roles: admin, cliente, transportista, operador

#### 2.4 Rutas CRUD de Usuarios (`api/routes/users.py`)
- ✅ **GET /users/me**
  - Obtiene información del usuario autenticado
  - Requiere token Bearer válido

- ✅ **GET /users/{user_id}**
  - Obtiene usuario por ID
  - Admin: puede ver cualquier usuario
  - Usuario normal: solo puede ver su propia info

- ✅ **PUT /users/{user_id}**
  - Actualiza datos de usuario
  - Admin: puede actualizar cualquier usuario
  - Usuario normal: solo puede actualizar su propia info
  - Solo admin puede cambiar estado (activo/inactivo/suspendido)

- ✅ **DELETE /users/{user_id}**
  - Elimina usuario (solo admin)

- ✅ **GET /users/**
  - Lista usuarios (solo admin)
  - Soporte para paginación

#### 2.5 Rutas CRUD de Pedidos (`api/routes/pedidos.py`)
- ✅ **POST /pedidos/**
  - Crea nuevo pedido
  - Solo clientes y admin pueden crear
  - Estado inicial: "pendiente"
  - Valida prioridad (normal, alta, urgente)

- ✅ **GET /pedidos/{pedido_id}**
  - Obtiene pedido por ID
  - Control de acceso:
    - Admin: todos los pedidos
    - Cliente: solo sus pedidos
    - Transportista: solo pedidos asignados

- ✅ **GET /pedidos/tracking/{numero_tracking}**
  - Obtiene pedido por número de tracking
  - Disponible para todos los usuarios autenticados

- ✅ **GET /pedidos/**
  - Lista pedidos según rol:
    - Admin: pedidos pendientes
    - Cliente: sus propios pedidos
    - Transportista: pedidos asignados
  - Soporte para paginación (skip/limit)

- ✅ **PUT /pedidos/{pedido_id}**
  - Actualiza pedido
  - Admin: puede actualizar todo
  - Transportista: puede actualizar estado de pedidos asignados
  - Soporta:
    - Cambio de estado
    - Asignación de transportista (solo admin)
    - Cambio de fecha estimada (solo admin)

- ✅ **DELETE /pedidos/{pedido_id}**
  - Elimina pedido (solo admin)

### 3. Aplicación Principal

#### 3.1 FastAPI App (`main.py`)
- ✅ Configuración completa de FastAPI
- ✅ CORS habilitado (configurar para producción)
- ✅ Creación automática de tablas en BD
- ✅ Documentación automática en `/docs` (Swagger UI)
- ✅ Documentación alternativa en `/redoc` (ReDoc)
- ✅ Endpoints de salud:
  - `GET /`: Información de la API
  - `GET /health`: Health check

- ✅ Routers incluidos:
  - `/auth/*`: Autenticación
  - `/users/*`: Gestión de usuarios
  - `/pedidos/*`: Gestión de pedidos

### 4. Actualizaciones de Exports
- ✅ `infrastructure/persistence/models/__init__.py`: Exporta UserDB
- ✅ `infrastructure/persistence/mappers/__init__.py`: Exporta UserMapper
- ✅ `infrastructure/persistence/__init__.py`: Exporta repositorio de usuarios

## 📋 Casos de Uso Implementados

### CU-001: Registro de Usuario
1. Usuario envía datos de registro (email, password, nombre, apellido, rol)
2. Sistema valida que email no exista
3. Sistema valida rol válido
4. Sistema hashea la contraseña
5. Sistema crea usuario con estado "activo"
6. Sistema genera token JWT
7. Sistema retorna token y datos de usuario

### CU-002: Login
1. Usuario envía credenciales (email, password)
2. Sistema busca usuario por email
3. Sistema verifica contraseña hasheada
4. Sistema valida que usuario esté activo
5. Sistema genera token JWT con datos del usuario
6. Sistema retorna token y datos de usuario

### CU-003: Crear Pedido
1. Cliente autenticado envía datos de pedido
2. Sistema valida permisos (cliente o admin)
3. Sistema valida prioridad
4. Sistema crea pedido con estado "pendiente"
5. Sistema asigna timestamp de solicitud
6. Sistema guarda pedido en BD
7. Sistema retorna datos del pedido creado

### CU-004: Consultar Pedido
1. Usuario autenticado solicita pedido por ID o tracking
2. Sistema verifica permisos:
   - Admin: acceso total
   - Cliente: solo sus pedidos
   - Transportista: solo asignados
3. Sistema retorna datos del pedido

### CU-005: Actualizar Pedido
1. Usuario autenticado solicita actualización
2. Sistema verifica permisos según rol
3. Admin puede:
   - Cambiar estado
   - Asignar transportista
   - Cambiar fecha estimada
4. Transportista puede:
   - Actualizar estado de pedidos asignados
5. Sistema guarda cambios
6. Sistema retorna pedido actualizado

### CU-006: Listar Pedidos
1. Usuario autenticado solicita lista
2. Sistema filtra según rol:
   - Admin: pendientes
   - Cliente: propios
   - Transportista: asignados
3. Sistema aplica paginación
4. Sistema retorna lista de pedidos

## 🔐 Seguridad Implementada

### Autenticación
- ✅ JWT con expiración de 30 minutos
- ✅ Hash de contraseñas con bcrypt (10 rounds)
- ✅ Token Bearer en header Authorization
- ✅ Validación de token en cada request protegido

### Autorización
- ✅ Control de acceso basado en roles (RBAC)
- ✅ Verificación de permisos por endpoint
- ✅ Usuarios solo acceden a sus propios recursos
- ✅ Admin tiene acceso completo

### Validación
- ✅ Email único en registro
- ✅ Usuario activo para login
- ✅ Validación de enums (roles, estados, prioridades)
- ✅ Validación de UUIDs
- ✅ Validación de tipos con Pydantic

## 🛠️ Stack Tecnológico Utilizado

### Backend
- **FastAPI**: Framework web moderno y rápido
- **SQLAlchemy**: ORM para base de datos
- **Pydantic**: Validación de datos y serialización
- **Passlib**: Hash de contraseñas
- **Python-Jose**: Manejo de JWT
- **Uvicorn**: Servidor ASGI

### Base de Datos
- **SQLite**: Base de datos para desarrollo (cambiar a PostgreSQL en producción)

### Arquitectura
- **Clean Architecture**: Separación de capas
- **Patrón Repository**: Abstracción de persistencia
- **Patrón Mapper**: Conversión entre modelos
- **Dependency Injection**: Usando FastAPI Depends

## 📊 Estructura de Archivos Creados

```
CourierBackend-proyectoSistemas/
├── api/
│   ├── __init__.py
│   ├── dependencies.py          # Inyección de dependencias
│   ├── schemas/
│   │   ├── __init__.py
│   │   ├── auth_schemas.py      # Esquemas de autenticación
│   │   ├── user_schemas.py      # Esquemas de usuario
│   │   └── pedido_schemas.py    # Esquemas de pedido
│   └── routes/
│       ├── __init__.py
│       ├── auth.py              # Rutas de autenticación
│       ├── users.py             # CRUD de usuarios
│       └── pedidos.py           # CRUD de pedidos
├── infrastructure/
│   ├── auth/
│   │   ├── __init__.py
│   │   └── security.py          # Servicios de seguridad
│   └── persistence/
│       ├── models/
│       │   ├── user_db.py       # Modelo SQLAlchemy Usuario
│       │   └── pedido_db.py     # (existente)
│       ├── mappers/
│       │   ├── user_mapper.py   # Mapper Usuario
│       │   └── pedido_mapper.py # (existente)
│       └── user_repository_sqlalchemy.py  # Repositorio Usuario
├── main.py                      # Aplicación FastAPI principal
└── daily-scrum2.md             # Esta documentación
```

## 🧪 Testing Manual

### Probar el servidor
```bash
# Iniciar servidor
cd /home/runner/work/CourierBackend-proyectoSistemas/CourierBackend-proyectoSistemas
python main.py

# O con uvicorn
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

### Endpoints disponibles
- Docs interactivos: http://localhost:8000/docs
- ReDoc: http://localhost:8000/redoc
- Health check: http://localhost:8000/health

### Ejemplos de uso con curl

#### 1. Registrar usuario
```bash
curl -X POST http://localhost:8000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@courier.com",
    "password": "admin123",
    "nombre": "Admin",
    "apellido": "Sistema",
    "role": "admin"
  }'
```

#### 2. Login
```bash
curl -X POST http://localhost:8000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@courier.com",
    "password": "admin123"
  }'
```

#### 3. Obtener información de usuario (con token)
```bash
TOKEN="tu-token-jwt-aqui"
curl -X GET http://localhost:8000/users/me \
  -H "Authorization: Bearer $TOKEN"
```

#### 4. Crear pedido
```bash
curl -X POST http://localhost:8000/pedidos/ \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "numero_tracking": "TRK-001",
    "id_cliente": "uuid-del-cliente",
    "fecha_entrega_estimada": "2025-10-20T10:00:00",
    "direccion_origen": "uuid-direccion-origen",
    "direccion_destino": "uuid-direccion-destino",
    "prioridad": "normal",
    "peso": 5.5,
    "monto_total": 150.00
  }'
```

#### 5. Listar pedidos
```bash
curl -X GET http://localhost:8000/pedidos/ \
  -H "Authorization: Bearer $TOKEN"
```

#### 6. Obtener pedido por tracking
```bash
curl -X GET http://localhost:8000/pedidos/tracking/TRK-001 \
  -H "Authorization: Bearer $TOKEN"
```

## 🔄 Flujo de Autenticación

```
1. Usuario → POST /auth/register o /auth/login
   ↓
2. API valida credenciales
   ↓
3. API genera token JWT
   ↓
4. Usuario recibe token
   ↓
5. Usuario envía token en header: Authorization: Bearer <token>
   ↓
6. Middleware get_current_user() valida token
   ↓
7. Request procesado si token válido
```

## 📈 Métricas de Implementación

- **Archivos creados**: 15
- **Líneas de código**: ~1,500
- **Endpoints implementados**: 13
- **Modelos de dominio utilizados**: 2 (User, Pedido)
- **Repositorios implementados**: 2
- **Casos de uso documentados**: 6

## 🚀 Próximos Pasos Sugeridos

1. **Testing**
   - Crear tests unitarios para servicios de auth
   - Crear tests de integración para endpoints
   - Implementar tests de autorización

2. **Mejoras de Seguridad**
   - Mover SECRET_KEY a variables de entorno
   - Implementar refresh tokens
   - Agregar rate limiting
   - Implementar blacklist de tokens

3. **Base de Datos**
   - Migrar de SQLite a PostgreSQL
   - Implementar migraciones con Alembic
   - Agregar índices adicionales

4. **Funcionalidades**
   - Recuperación de contraseña
   - Verificación de email
   - Paginación mejorada con cursores
   - Filtros avanzados en listados

5. **Documentación**
   - Agregar docstrings completos
   - Crear Postman collection
   - Documentar variables de entorno
   - Crear guía de deployment

## 📝 Notas Técnicas

### Patrón de Doble Modelo
Se mantiene consistente con la arquitectura existente:
- **Entidad de Dominio (Pydantic)**: User, Pedido - Agnósticas a tecnología
- **Modelo de Persistencia (SQLAlchemy)**: UserDB, PedidoDB - Específicos de BD
- **Mapper**: Conversión bidireccional entre modelos

### Inyección de Dependencias
FastAPI Depends se usa para:
- Gestión de sesiones de BD
- Obtención de repositorios
- Autenticación de usuarios
- Verificación de roles

### Control de Acceso
Implementado mediante decoradores:
```python
current_user: User = Depends(get_current_user)  # Requiere auth
admin_user: User = Depends(require_admin)       # Requiere admin
```

## ✨ Conclusión

Se ha implementado exitosamente un sistema completo de autenticación y CRUD básico para el Sistema Courier, siguiendo las mejores prácticas de Clean Architecture y manteniendo consistencia con el código existente. El sistema está listo para desarrollo y testing, con documentación automática disponible en `/docs`.
