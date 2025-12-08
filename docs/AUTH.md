# 🔐 Sistema de Autenticación

## Estrategia

El proyecto usa **Supabase Auth** como proveedor de identidad. El frontend maneja login/registro directamente con Supabase, y el backend Rust **solo valida** los tokens JWT.

## Flujo de Autenticación

```
┌──────────────┐     ┌─────────────────┐     ┌──────────────────┐
│   Frontend   │     │  Supabase Auth  │     │  Backend Rust    │
│  (React/Vue) │     │                 │     │                  │
└──────┬───────┘     └────────┬────────┘     └────────┬─────────┘
       │                      │                       │
       │  1. Login (email/pass)                       │
       │─────────────────────▶│                       │
       │                      │                       │
       │  2. JWT Token        │                       │
       │◀─────────────────────│                       │
       │                      │                       │
       │  3. Request + Bearer Token                   │
       │─────────────────────────────────────────────▶│
       │                      │                       │
       │                      │  4. Validar JWT       │
       │                      │  (con JWT_SECRET)     │
       │                      │                       │
       │  5. Response (datos protegidos)              │
       │◀─────────────────────────────────────────────│
```

## Responsabilidades

### Frontend (Supabase SDK)
- Login con email/password
- Login social (Google, GitHub, etc.)
- Registro de usuarios
- Refresh de tokens
- Almacenar token en memoria/localStorage

### Backend Rust
- Validar JWT en cada request protegido
- Extraer `user_id` del token
- Verificar roles y permisos
- Rechazar tokens inválidos o expirados

## Configuración en Frontend

```javascript
import { createClient } from '@supabase/supabase-js'

const supabase = createClient('TU_SUPABASE_URL', 'TU_ANON_KEY')

// Login
const { data, error } = await supabase.auth.signInWithPassword({
  email: 'usuario@email.com',
  password: 'contraseña123'
})

// Obtener token
const token = data.session.access_token

// Llamar al backend Rust
const response = await fetch('http://localhost:3000/api/pedidos', {
  headers: {
    'Authorization': `Bearer ${token}`
  }
})
```

## Configuración en Backend

### Variables de Entorno
```env
SUPABASE_URL=https://tu-proyecto.supabase.co
SUPABASE_JWT_SECRET=tu-jwt-secret-de-supabase
```

### Obtener credenciales de Supabase
1. Ir a **Supabase Dashboard** → tu proyecto
2. **Settings** → **API**
3. Copiar:
   - `Project URL` → `SUPABASE_URL`
   - `JWT Secret` → `SUPABASE_JWT_SECRET`

## Claims del JWT de Supabase

```json
{
  "aud": "authenticated",
  "exp": 1702000000,
  "iat": 1701900000,
  "iss": "https://tu-proyecto.supabase.co/auth/v1",
  "sub": "uuid-del-usuario",
  "email": "usuario@email.com",
  "role": "authenticated",
  "app_metadata": {
    "provider": "email"
  },
  "user_metadata": {
    "nombre": "Juan"
  }
}
```

## Roles y Permisos

Los roles se definen en la tabla `users.rol`:
- `cliente`: Acceso a pedidos propios
- `transportista`: Acceso a pedidos asignados
- `admin`: Acceso completo

## Endpoints del Backend

| Método | Endpoint | Descripción | Auth |
|--------|----------|-------------|------|
| GET | `/auth/me` | Usuario actual | ✅ |
| GET | `/health` | Health check | ❌ |

> **Nota**: Login y registro se manejan directamente con Supabase desde el frontend.

## Protección de Rutas en Rust

```rust
// Rutas protegidas (requieren JWT válido)
Router::new()
    .route("/api/pedidos", get(get_pedidos))
    .route("/auth/me", get(get_current_user))
    .route_layer(middleware::from_fn(require_auth))

// Rutas públicas
Router::new()
    .route("/health", get(health_check))
```

## Manejo de Errores

| Código | Error | Causa |
|--------|-------|-------|
| 401 | `TOKEN_MISSING` | No se envió header Authorization |
| 401 | `TOKEN_INVALID` | JWT malformado o firma inválida |
| 401 | `TOKEN_EXPIRED` | JWT expirado |
| 403 | `FORBIDDEN` | Sin permisos para el recurso |
