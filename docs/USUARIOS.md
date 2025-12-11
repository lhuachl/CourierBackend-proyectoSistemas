# 👤 Módulo de Usuarios

## Descripción General

El módulo de usuarios gestiona la información extendida de usuarios del sistema. A diferencia de Supabase `auth.users` que maneja autenticación, la tabla `public.users` almacena datos administrativos y de negocio específicos del sistema.

## Arquitectura

```
Presentation (handlers)
    ↓
Application (services + DTOs)
    ↓
Domain (entities + repositories)
    ↓
Infrastructure (repository implementations + SQL queries)
    ↓
Database (PostgreSQL)
```

## Entidad User

### Estructura

```rust
pub struct User {
    pub id: Uuid,                        // ID único del usuario
    pub email: Option<String>,           // Email sincronizado desde Supabase
    pub nombre: Option<String>,          // Nombre del usuario
    pub apellido: Option<String>,        // Apellido del usuario
    pub rol: String,                     // Rol: cliente, transportista, admin
    pub foto_perfil: Option<String>,     // URL de foto de perfil
    pub activo: bool,                    // Estado (soft delete)
    pub created_at: DateTime<Utc>,       // Fecha de creación
    pub updated_at: DateTime<Utc>,       // Fecha de última actualización
}
```

### Roles Disponibles

| Rol | Descripción | Permisos |
|-----|-------------|----------|
| `cliente` | Usuario que realiza pedidos | Crear/ver pedidos propios, direcciones propias |
| `transportista` | Reparte pedidos | Ver pedidos asignados, actualizar estado |
| `admin` | Administrador del sistema | Acceso completo a todos los endpoints |

## DTOs (Data Transfer Objects)

### CreateUserDTO

```json
{
  "email": "usuario@email.com",
  "nombre": "Juan",
  "apellido": "Pérez",
  "rol": "cliente"
}
```

### UpdateUserDTO

```json
{
  "email": "nuevo@email.com",
  "nombre": "Juan",
  "apellido": "Pérez",
  "rol": "cliente",
  "foto_perfil": "https://..."
}
```

### UpdateUserRoleDTO

```json
{
  "rol": "transportista"
}
```

### UpdateUserStatusDTO

```json
{
  "activo": true
}
```

### UserResponseDTO

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "usuario@email.com",
  "nombre": "Juan",
  "apellido": "Pérez",
  "rol": "cliente",
  "foto_perfil": "https://...",
  "activo": true,
  "created_at": "2025-12-11T10:30:00Z",
  "updated_at": "2025-12-11T15:45:00Z"
}
```

## Endpoints

### Listar Usuarios

```http
GET /api/admin/users
Authorization: Bearer <token>
```

**Parámetros:** Ninguno

**Respuesta (200 OK):**
```json
{
  "total": 5,
  "users": [
    { /* UserResponseDTO */ }
  ]
}
```

**Errores:**
- `401`: No autenticado
- `403`: No es administrador
- `500`: Error interno del servidor

---

### Obtener Usuario por ID

```http
GET /api/admin/users/{id}
Authorization: Bearer <token>
```

**Parámetros:**
- `id` (UUID, path): ID del usuario a obtener

**Respuesta (200 OK):**
```json
{ /* UserResponseDTO */ }
```

**Errores:**
- `401`: No autenticado
- `403`: No es administrador
- `404`: Usuario no encontrado
- `500`: Error interno del servidor

---

### Crear Usuario

```http
POST /api/admin/users
Authorization: Bearer <token>
Content-Type: application/json

{
  "email": "nuevo@email.com",
  "nombre": "Nuevo",
  "apellido": "Usuario",
  "rol": "cliente"
}
```

**Parámetros:**
- Body: `CreateUserDTO`

**Respuesta (201 Created):**
```json
{ /* UserResponseDTO */ }
```

**Errores:**
- `400`: Email ya existe o datos inválidos
- `401`: No autenticado
- `403`: No es administrador
- `500`: Error interno del servidor

---

### Actualizar Usuario

```http
PUT /api/admin/users/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "email": "actualizado@email.com",
  "nombre": "Actualizado",
  "apellido": "Usuario",
  "rol": "transportista",
  "foto_perfil": "https://..."
}
```

**Parámetros:**
- `id` (UUID, path): ID del usuario a actualizar
- Body: `UpdateUserDTO`

**Respuesta (200 OK):**
```json
{ /* UserResponseDTO actualizado */ }
```

**Errores:**
- `400`: Datos inválidos
- `401`: No autenticado
- `403`: No es administrador
- `404`: Usuario no encontrado
- `500`: Error interno del servidor

---

### Actualizar Rol de Usuario

```http
PATCH /api/admin/users/{id}/role
Authorization: Bearer <token>
Content-Type: application/json

{
  "rol": "admin"
}
```

**Parámetros:**
- `id` (UUID, path): ID del usuario a actualizar
- Body: `UpdateUserRoleDTO`

**Respuesta (200 OK):**
```json
{ /* UserResponseDTO con rol actualizado */ }
```

**Errores:**
- `400`: Rol inválido
- `401`: No autenticado
- `403`: No es administrador
- `404`: Usuario no encontrado
- `500`: Error interno del servidor

---

### Actualizar Estado de Usuario

```http
PATCH /api/admin/users/{id}/status
Authorization: Bearer <token>
Content-Type: application/json

{
  "activo": false
}
```

**Parámetros:**
- `id` (UUID, path): ID del usuario a actualizar
- Body: `UpdateUserStatusDTO`

**Respuesta (200 OK):**
```json
{ /* UserResponseDTO con estado actualizado */ }
```

**Errores:**
- `401`: No autenticado
- `403`: No es administrador
- `404`: Usuario no encontrado
- `500`: Error interno del servidor

---

### Eliminar Usuario (Soft Delete)

```http
DELETE /api/admin/users/{id}
Authorization: Bearer <token>
```

**Parámetros:**
- `id` (UUID, path): ID del usuario a eliminar

**Respuesta (204 No Content)**

**Errores:**
- `401`: No autenticado
- `403`: No es administrador
- `404`: Usuario no encontrado
- `500`: Error interno del servidor

## Sincronización con Supabase

El flujo de sincronización es el siguiente:

1. **Usuario registra en Supabase** (a través de SDK en frontend)
   - Se crea en `auth.users` de Supabase
   - Se obtiene JWT

2. **Webhook de Supabase** (pendiente de implementar)
   - Notifica al backend cuando se crea un usuario
   - Backend crea automáticamente el registro en `public.users`
   - Asigna rol por defecto: `cliente`

3. **Alternativa Manual** (actualmente)
   - Admin crea usuario manualmente vía `POST /api/admin/users`
   - Proporciona email y datos básicos

### Implementación de Webhook (Recomendada)

```javascript
// Configurar en Supabase Dashboard
// Settings → Functions

export async function handleAuthWebhook(req) {
  const { type, data } = req.body;
  
  if (type === 'user_created') {
    const response = await fetch('http://tu-backend/api/admin/users', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${ADMIN_TOKEN}`
      },
      body: JSON.stringify({
        email: data.email,
        nombre: data.user_metadata?.nombre,
        apellido: data.user_metadata?.apellido,
        rol: 'cliente'
      })
    });
    
    return response.json();
  }
}
```

## Ejemplos de Uso

### Listar todos los usuarios

```bash
curl -H "Authorization: Bearer {token}" \
  http://localhost:3000/api/admin/users
```

### Crear un usuario

```bash
curl -X POST \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "juan@example.com",
    "nombre": "Juan",
    "apellido": "Pérez",
    "rol": "cliente"
  }' \
  http://localhost:3000/api/admin/users
```

### Cambiar rol de usuario a admin

```bash
curl -X PATCH \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{"rol": "admin"}' \
  http://localhost:3000/api/admin/users/{user_id}/role
```

### Suspender un usuario

```bash
curl -X PATCH \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{"activo": false}' \
  http://localhost:3000/api/admin/users/{user_id}/status
```

## Validaciones

- **Email**: Único, formato válido de email
- **Rol**: Debe ser uno de: `cliente`, `transportista`, `admin`
- **Nombre/Apellido**: Opcional, máx 255 caracteres
- **Foto Perfil**: URL válida (opcional)

## Seguridad

- ✅ Todos los endpoints requieren autenticación JWT
- ✅ Solo admins pueden acceder a los endpoints de usuarios
- ✅ Soft delete preserva historial de datos
- ✅ Campos sensibles no se exponen en respuestas
- ⚠️ TODO: Implementar rate limiting
- ⚠️ TODO: Implementar audit logging

## Notas

- El campo `email` se sincroniza desde Supabase y es único
- La eliminación es soft delete (campo `activo = false`)
- Los usuarios inactivos no aparecen en listados
- Los cambios de rol se efectúan inmediatamente
