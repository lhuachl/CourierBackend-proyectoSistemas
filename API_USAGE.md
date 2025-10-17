# API Usage Guide - Sistema Courier

## 🚀 Inicio Rápido

### Requisitos
- Python 3.12+
- pip

### Instalación

```bash
# Instalar dependencias
pip install fastapi sqlalchemy pydantic "pydantic[email]" passlib python-jose python-multipart uvicorn python-dotenv

# Configurar variables de entorno
cp .env.example .env
# Editar .env con tus valores (ver ENV.md para detalles)

# Iniciar el servidor
python main.py
```

El servidor estará disponible en: http://localhost:8000

## 📚 Documentación Interactiva

- **Swagger UI**: http://localhost:8000/docs
- **ReDoc**: http://localhost:8000/redoc

## 🔐 Autenticación

### Registro de Usuario

```bash
curl -X POST http://localhost:8000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "usuario@example.com",
    "password": "password123",
    "nombre": "Nombre",
    "apellido": "Apellido",
    "role": "cliente"
  }'
```

**Roles disponibles:** `admin`, `cliente`, `transportista`, `operador`

**Respuesta:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5...",
  "token_type": "bearer",
  "user_id": "c990a2f7-04af-4ea3-983f-c4321c55fb1b",
  "email": "usuario@example.com",
  "role": "cliente"
}
```

### Login

```bash
curl -X POST http://localhost:8000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "usuario@example.com",
    "password": "password123"
  }'
```

**Respuesta:** Igual que el registro, incluye el `access_token`.

## 👤 Endpoints de Usuarios

### Obtener Información del Usuario Actual

```bash
curl -X GET http://localhost:8000/users/me \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Obtener Usuario por ID

```bash
curl -X GET http://localhost:8000/users/{user_id} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Actualizar Usuario

```bash
curl -X PUT http://localhost:8000/users/{user_id} \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Nuevo Nombre",
    "apellido": "Nuevo Apellido"
  }'
```

**Nota:** Solo admin puede cambiar el estado de un usuario.

### Eliminar Usuario (Solo Admin)

```bash
curl -X DELETE http://localhost:8000/users/{user_id} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 📦 Endpoints de Pedidos

### Crear Pedido

```bash
curl -X POST http://localhost:8000/pedidos/ \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "numero_tracking": "TRK-001",
    "id_cliente": "USER_UUID",
    "fecha_entrega_estimada": "2025-10-20T10:00:00",
    "direccion_origen": "ORIGEN_UUID",
    "direccion_destino": "DESTINO_UUID",
    "prioridad": "normal",
    "peso": 5.5,
    "monto_total": 150.00
  }'
```

**Prioridades disponibles:** `normal`, `alta`, `urgente`

### Obtener Pedido por ID

```bash
curl -X GET http://localhost:8000/pedidos/{pedido_id} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Obtener Pedido por Número de Tracking

```bash
curl -X GET http://localhost:8000/pedidos/tracking/TRK-001 \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Listar Pedidos

```bash
curl -X GET http://localhost:8000/pedidos/ \
  -H "Authorization: Bearer YOUR_TOKEN"
```

**Filtrado automático por rol:**
- **Admin:** Ve todos los pedidos pendientes
- **Cliente:** Ve solo sus propios pedidos
- **Transportista:** Ve pedidos asignados a él

### Actualizar Pedido

```bash
curl -X PUT http://localhost:8000/pedidos/{pedido_id} \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "estado": "en_ruta",
    "id_transportista": "TRANSPORTISTA_UUID"
  }'
```

**Estados disponibles:** `pendiente`, `procesando`, `en_ruta`, `entregado`, `cancelado`

**Permisos:**
- **Admin:** Puede actualizar todo (estado, transportista, fecha estimada)
- **Transportista:** Puede actualizar estado de pedidos asignados

### Eliminar Pedido (Solo Admin)

```bash
curl -X DELETE http://localhost:8000/pedidos/{pedido_id} \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 🔒 Control de Acceso

### Matriz de Permisos

| Endpoint | Admin | Cliente | Transportista | Operador |
|----------|-------|---------|---------------|----------|
| POST /auth/register | ✅ | ✅ | ✅ | ✅ |
| POST /auth/login | ✅ | ✅ | ✅ | ✅ |
| GET /users/me | ✅ | ✅ | ✅ | ✅ |
| GET /users/{id} | ✅ (todos) | ✅ (solo propio) | ✅ (solo propio) | ✅ (solo propio) |
| PUT /users/{id} | ✅ (todos) | ✅ (solo propio) | ✅ (solo propio) | ✅ (solo propio) |
| DELETE /users/{id} | ✅ | ❌ | ❌ | ❌ |
| GET /users/ | ✅ | ❌ | ❌ | ❌ |
| POST /pedidos/ | ✅ | ✅ | ❌ | ❌ |
| GET /pedidos/ | ✅ (todos) | ✅ (propios) | ✅ (asignados) | ❌ |
| GET /pedidos/{id} | ✅ | ✅ (propios) | ✅ (asignados) | ❌ |
| PUT /pedidos/{id} | ✅ (todo) | ❌ | ✅ (estado) | ❌ |
| DELETE /pedidos/{id} | ✅ | ❌ | ❌ | ❌ |

## 🧪 Ejemplo de Flujo Completo

```bash
# 1. Registrar cliente
REGISTER_RESPONSE=$(curl -s -X POST http://localhost:8000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "cliente@test.com",
    "password": "cliente123",
    "nombre": "Cliente",
    "apellido": "Prueba",
    "role": "cliente"
  }')

# Extraer token y user_id
TOKEN=$(echo $REGISTER_RESPONSE | python -c "import sys, json; print(json.load(sys.stdin)['access_token'])")
USER_ID=$(echo $REGISTER_RESPONSE | python -c "import sys, json; print(json.load(sys.stdin)['user_id'])")

echo "Token: $TOKEN"
echo "User ID: $USER_ID"

# 2. Obtener información del usuario
curl -X GET http://localhost:8000/users/me \
  -H "Authorization: Bearer $TOKEN"

# 3. Crear un pedido
curl -X POST http://localhost:8000/pedidos/ \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"numero_tracking\": \"TRK-TEST-$(date +%s)\",
    \"id_cliente\": \"$USER_ID\",
    \"fecha_entrega_estimada\": \"2025-10-20T10:00:00\",
    \"direccion_origen\": \"11111111-1111-1111-1111-111111111111\",
    \"direccion_destino\": \"22222222-2222-2222-2222-222222222222\",
    \"prioridad\": \"normal\",
    \"peso\": 5.5,
    \"monto_total\": 150.00
  }"

# 4. Listar pedidos
curl -X GET http://localhost:8000/pedidos/ \
  -H "Authorization: Bearer $TOKEN"
```

## ⚙️ Configuración

### Variables de Entorno
El proyecto ahora utiliza variables de entorno para la configuración. Ver **[ENV.md](ENV.md)** para documentación completa.

**Configuración rápida:**

1. Copiar el archivo de ejemplo:
```bash
cp .env.example .env
```

2. Editar `.env` con tus valores:
```bash
DATABASE_URL=postgresql://postgres:your_password@db.your_project.supabase.co:5432/postgres
SECRET_KEY=your-secret-key-here
```

3. Generar una SECRET_KEY segura:
```bash
python -c "import secrets; print(secrets.token_urlsafe(32))"
```

### Base de Datos
Por defecto, la aplicación usa SQLite (`courier.db`). Para usar PostgreSQL/Supabase:

1. Configurar `DATABASE_URL` en el archivo `.env`:
```
DATABASE_URL=postgresql://user:password@host:port/database
```

2. Instalar driver de PostgreSQL:
```bash
pip install psycopg2-binary
```

Ver [ENV.md](ENV.md) para instrucciones detalladas de configuración con Supabase.

### CORS
Configurar orígenes permitidos en `main.py`:

```python
app.add_middleware(
    CORSMiddleware,
    allow_origins=["https://tu-frontend.com"],  # Especificar dominio
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
```

## 🐛 Solución de Problemas

### Error: "Token inválido o expirado"
- Los tokens JWT expiran en 30 minutos
- Realizar login nuevamente para obtener un token nuevo

### Error: "Credenciales incorrectas"
- Verificar que el email esté registrado
- Verificar que la contraseña sea correcta
- Los emails son case-sensitive

### Error: "El email ya está registrado"
- El email ya existe en la base de datos
- Usar un email diferente o hacer login con el existente

### Error: "No tienes permisos"
- Verificar que el token sea válido
- Verificar que el usuario tenga el rol correcto
- Admin puede acceder a todos los recursos

## 📖 Documentación Adicional

- Ver `daily-scrum2.md` para detalles de implementación
- Ver documentación interactiva en `/docs`
- Ver código de ejemplo en `infrastructure/persistence/example_usage.py`
