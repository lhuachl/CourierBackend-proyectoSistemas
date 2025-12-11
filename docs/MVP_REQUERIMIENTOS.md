# 📋 Requerimientos MVP - Integrador Backend

## Objetivo del MVP

Crear una plataforma de logística y entregas funcional que permita a clientes crear pedidos, transportistas entregarlos, y administradores gestionar la operación.

**Fecha de Actualización:** 11 de diciembre de 2025

## Tabla de Contenidos
1. [Definición de Roles](#definición-de-roles-y-alcance)
2. [Flujo de Autenticación y Registro](#flujo-de-autenticación-y-registro)
3. [Módulo de Login del Frontend](#módulo-de-login-del-frontend)
4. [Requisitos Funcionales](#requisitos-funcionales)
5. [Endpoints Implementados](#endpoints-implementados)
6. [Criterios de Aceptación](#criterios-de-aceptación)

---

## Flujo de Autenticación y Registro

### Arquitectura Actual: Autenticación Delegada

El sistema utiliza **Supabase Auth** como proveedor de identidad externo:

```
┌──────────────────┐        ┌───────────────┐        ┌──────────────────┐
│  Frontend (App)  │        │  Supabase     │        │  Backend Rust    │
│                  │        │  Auth         │        │                  │
└────────┬─────────┘        └───────┬───────┘        └────────┬─────────┘
         │                          │                         │
         │  1. Sign Up / Login      │                         │
         │─────────────────────────►│                         │
         │                          │                         │
         │  2. JWT Token            │                         │
         │◄─────────────────────────│                         │
         │                          │                         │
         │  3. API Request + JWT    │                         │
         │──────────────────────────────────────────────────►│
         │                          │                         │
         │                          │  4. Validate JWT       │
         │                          │  (check signature)     │
         │                          │                         │
         │  5. Response             │                         │
         │◄──────────────────────────────────────────────────│
```

**Ventajas:**
- No mantenemos contraseñas
- Supabase maneja actualizaciones de seguridad
- Social login integrado
- MFA disponible

### Proceso de Registro Actual

#### Paso 1: Registro en Supabase (Frontend)
```javascript
// El frontend registra al usuario en Supabase
const { data, error } = await supabase.auth.signUp({
  email: 'usuario@ejemplo.com',
  password: 'MiContraseña123',
  options: {
    data: {
      nombre: 'Juan',
      apellido: 'Pérez'
    }
  }
});

// Resultado:
// ✅ Usuario creado en auth.users (Supabase)
// ❌ No existe aún en public.users (nuestro backend)
```

#### Paso 2: Crear Usuario en Backend (INTERVENCIÓN MANUAL)
```javascript
// Frontend DEBE hacer una llamada adicional
const token = data.session.access_token;

const response = await fetch('http://localhost:3000/api/admin/users', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    email: 'usuario@ejemplo.com',
    nombre: 'Juan',
    apellido: 'Pérez',
    rol: 'cliente'
  })
});

// Resultado:
// ✅ Usuario creado en public.users (nuestro backend)
// ✅ Rol asignado como 'cliente'
```

#### Paso 3: Crear Perfil de Cliente (INTERVENCIÓN MANUAL)
```javascript
// Frontend DEBE hacer otra llamada adicional
const perfilResponse = await fetch('http://localhost:3000/api/perfil', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    nombre: 'Juan',
    telefono: '+1234567890',
    documento: '12345678'
  })
});

// Resultado:
// ✅ Perfil de cliente creado
// ✅ Usuario listo para crear pedidos
```

### Problema: ¿Por Qué "Intervención Manual"?

La intervención manual se refiere a que el frontend debe ejecutar **3 pasos separados y secuenciales**:

1. **Supabase SDK** (`supabase.auth.signUp()`) - Crea autenticación
2. **Backend API** (`POST /api/admin/users`) - Crea datos de usuario
3. **Backend API** (`POST /api/perfil`) - Crea perfil de cliente

**Si el frontend no ejecuta todos los pasos:**
- ❌ Usuario existe en Supabase pero no en nuestro backend
- ❌ No puede acceder a `GET /api/perfil`
- ❌ No puede crear pedidos
- ❌ Estado inconsistente

**Flujo Problemático Actual:**
```
Usuario ingresa credenciales
         ↓
Frontend llama supabase.auth.signUp() ✅
         ↓
¿El frontend llama a POST /api/admin/users?
    ├─ ✅ SÍ → Continúa
    └─ ❌ NO → PROBLEMA: Usuario atrapado sin datos de backend
         ↓
¿El frontend llama a POST /api/perfil?
    ├─ ✅ SÍ → Usuario listo ✅
    └─ ❌ NO → PROBLEMA: Usuario sin perfil, no puede crear pedidos
```

### Solución Propuesta para Fase 2

Implementar **Webhook de Supabase** para sincronización automática:

```
Usuario se registra en Supabase
         ↓
Supabase envía webhook al backend
         ↓
Backend crea automáticamente:
  1. Registro en public.users
  2. Registro en perfiles_cliente (con datos del webhook)
         ↓
Usuario listo sin intervención manual ✅
```

---

## Módulo de Login del Frontend

### Componentes Necesarios

El frontend debe implementar un módulo de autenticación que gestione:

#### 1. Login (Supabase + Backend Sync)
```typescript
async function handleLogin(email: string, password: string) {
  // 1. Autenticar en Supabase
  const { data, error } = await supabase.auth.signInWithPassword({
    email,
    password
  });

  if (error) {
    showError('Email o contraseña incorrectos');
    return;
  }

  const token = data.session.access_token;

  // 2. Obtener datos actuales del usuario desde backend
  const userResponse = await fetch('/auth/me', {
    headers: { 'Authorization': `Bearer ${token}` }
  });

  if (!userResponse.ok) {
    showError('Error obteniendo datos del usuario');
    return;
  }

  const user = await userResponse.json();

  // 3. Guardar en estado global
  setCurrentUser(user);
  localStorage.setItem('token', token);

  // 4. Redirigir al dashboard
  navigate('/dashboard');
}
```

#### 2. Registro (Multi-paso)
```typescript
async function handleRegister(formData: RegistrationForm) {
  try {
    // PASO 1: Registrar en Supabase
    const { data, error } = await supabase.auth.signUp({
      email: formData.email,
      password: formData.password,
      options: {
        data: {
          nombre: formData.nombre,
          apellido: formData.apellido
        }
      }
    });

    if (error) throw new Error(error.message);

    const token = data.session?.access_token;
    if (!token) throw new Error('No se obtuvo token de autenticación');

    // PASO 2: Crear usuario en backend
    const userCreateResponse = await fetch('/api/admin/users', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        email: formData.email,
        nombre: formData.nombre,
        apellido: formData.apellido,
        rol: 'cliente'
      })
    });

    if (!userCreateResponse.ok) {
      throw new Error('Error creando usuario en el sistema');
    }

    // PASO 3: Crear perfil de cliente
    const profileResponse = await fetch('/api/perfil', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        nombre: formData.nombre,
        telefono: formData.telefono || ''
      })
    });

    if (!profileResponse.ok) {
      throw new Error('Error creando perfil de cliente');
    }

    // Todos los pasos completados exitosamente
    showSuccess('¡Registro exitoso! Bienvenido al sistema');
    localStorage.setItem('token', token);
    navigate('/dashboard');

  } catch (error) {
    showError(error.message);
  }
}
```

#### 3. Logout
```typescript
async function handleLogout() {
  // 1. Logout en Supabase
  await supabase.auth.signOut();

  // 2. Limpiar estado local
  clearCurrentUser();
  localStorage.removeItem('token');

  // 3. Redirigir a login
  navigate('/login');
}
```

#### 4. Mantener Sesión Activa
```typescript
function useAuth() {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Al cargar la app, restaurar sesión
    const token = localStorage.getItem('token');
    
    if (token) {
      // Validar que el token siga siendo válido
      fetch('/auth/me', {
        headers: { 'Authorization': `Bearer ${token}` }
      })
        .then(res => res.json())
        .then(data => setUser(data))
        .catch(() => {
          // Token expirado, limpiar
          localStorage.removeItem('token');
        })
        .finally(() => setLoading(false));
    } else {
      setLoading(false);
    }
  }, []);

  return { user, loading };
}
```

### Estructura de Carpetas Recomendada (Frontend)

```
src/
├── modules/
│   └── auth/
│       ├── components/
│       │   ├── LoginForm.tsx
│       │   ├── RegisterForm.tsx
│       │   ├── ProtectedRoute.tsx
│       │   └── LoadingScreen.tsx
│       ├── hooks/
│       │   └── useAuth.ts
│       ├── services/
│       │   ├── authService.ts
│       │   └── supabaseClient.ts
│       ├── store/
│       │   ├── authSlice.ts
│       │   └── types.ts
│       ├── pages/
│       │   ├── LoginPage.tsx
│       │   ├── RegisterPage.tsx
│       │   └── ForgotPasswordPage.tsx
│       └── index.ts
└── ...
```

---

## 1. Definición de Roles y Alcance

### Cliente
- ✅ Registrarse en el sistema (Supabase)
- ✅ Crear perfil con datos básicos
- ✅ Gestionar múltiples direcciones de entrega
- ✅ Crear pedidos
- ✅ Ver estado de sus pedidos
- ⚠️ Rastrear pedido en tiempo real (pendiente)
- ⚠️ Recibir notificaciones (pendiente)

### Transportista
- ⚠️ Registrarse/aprobarse como transportista
- ⚠️ Ver pedidos disponibles en su zona
- ⚠️ Aceptar/rechazar pedidos
- ⚠️ Actualizar estado de entrega (en tránsito, entregado)
- ⚠️ Ver historial de entregas
- ⚠️ Rastreo en tiempo real (pendiente)

### Administrador
- ✅ Gestionar usuarios (crear, editar, eliminar)
- ✅ Gestionar productos/catálogo
- ✅ Ver todos los pedidos
- ✅ Asignar transportistas a pedidos
- ✅ Cambiar roles de usuarios
- ⚠️ Ver reportes y estadísticas
- ⚠️ Gestionar zonas de cobertura
- ⚠️ Procesar pagos/facturas

## 2. Flujo Principal: Crear un Pedido

### Paso 1: Registro del Usuario

**Actor:** Cliente

**Tecnología:** Supabase Auth (frontend)

1. Cliente abre app
2. Hace clic en "Registrar"
3. Ingresa email, contraseña, nombre, teléfono
4. Confirma email
5. Supabase crea usuario en `auth.users`

**Estado Base de Datos:**
```
auth.users (Supabase)
├── id: uuid
├── email: usuario@email.com
├── encrypted_password: ...
└── user_metadata: { nombre, teléfono }
```

---

### Paso 2: Sincronización al Backend

**Actor:** Sistema (Webhook)

**Flujo Propuesto (futuro):**

1. Supabase dispara webhook al crear usuario
2. Backend recibe POST en endpoint `/webhooks/users/created`
3. Crea registro en `public.users` con rol `cliente`
4. Crea registro en `perfiles_cliente`

**Alternativa Actual (manual):**

1. Admin debe crear usuario manualmente:
```bash
POST /api/admin/users
{
  "email": "usuario@email.com",
  "nombre": "Juan",
  "apellido": "Pérez",
  "rol": "cliente"
}
```

**Estado Base de Datos:**
```
public.users
├── id: uuid
├── email: usuario@email.com
├── rol: "cliente"
├── activo: true
└── ...

perfiles_cliente
├── id_usuario: uuid (FK)
├── nombre: "Juan"
├── telefono: "+..."
└── ...
```

---

### Paso 3: Crear Perfil y Direcciones

**Actor:** Cliente (autenticado)

**Endpoint:** `POST /api/perfil`

```json
{
  "nombre": "Juan Pérez",
  "telefono": "+1234567890"
}
```

**Crear Dirección:** `POST /api/direcciones`

```json
{
  "calle": "Calle Principal 123",
  "ciudad": "Bogotá",
  "pais": "Colombia",
  "codigo_postal": "110111",
  "latitud": 4.7110,
  "longitud": -74.0721,
  "es_predeterminada": true
}
```

**Estado Base de Datos:**
```
perfiles_cliente
├── nombre: "Juan Pérez"
├── telefono: "+1234567890"
└── id_usuario: uuid

direcciones
├── id_perfil: uuid (FK)
├── calle: "Calle Principal 123"
├── ciudad: "Bogotá"
├── latitud: 4.7110
├── longitud: -74.0721
├── es_predeterminada: true
├── activo: true
└── ...
```

---

### Paso 4: Crear Pedido

**Actor:** Cliente (autenticado)

**Endpoint:** `POST /api/pedidos`

```json
{
  "id_producto": "uuid",
  "cantidad": 2,
  "id_direccion_entrega": "uuid",
  "notas": "Entregar en recepción"
}
```

**Validaciones:**
- Producto debe existir y estar activo
- Dirección debe pertenecer al cliente
- Stock disponible

**Respuesta (201 Created):**
```json
{
  "id": "uuid",
  "id_cliente": "uuid",
  "estado": "pendiente",
  "producto": { /* datos */ },
  "direccion_entrega": { /* datos */ },
  "fecha_creacion": "2025-12-11T10:30:00Z"
}
```

**Estado Base de Datos:**
```
pedidos
├── id: uuid
├── id_cliente: uuid (FK)
├── id_producto: uuid (FK)
├── cantidad: 2
├── id_direccion: uuid (FK)
├── estado: "pendiente"
├── total: 50000
├── created_at: 2025-12-11T10:30:00Z
└── ...
```

---

### Paso 5: Asignar Transportista (Admin)

**Actor:** Administrador

**Endpoint:** `PATCH /api/pedidos/{id}/transportista`

```json
{
  "id_transportista": "uuid"
}
```

**Estado cambios:**
```
pedidos
├── id_transportista: uuid (FK)
└── estado: "confirmado"
```

---

### Paso 6: Transportista Entrega

**Actor:** Transportista

**Endpoint:** `PATCH /api/pedidos/{id}/estado`

```json
{
  "estado": "entregado",
  "ubicacion_entrega": { "lat": 4.71, "lon": -74.07 },
  "firma_cliente": "data:image/png;base64,..."
}
```

**Estado Final:**
```
pedidos
├── estado: "entregado"
├── fecha_entrega: 2025-12-11T14:45:00Z
└── ...
```

---

## 3. Módulos Implementados vs Pendientes

### ✅ Implementados en MVP

| Módulo | Descripción | Endpoints | Status |
|--------|-------------|-----------|--------|
| **Auth** | Validación JWT | 1 endpoint | Básico |
| **Usuarios** | Gestión de usuarios (admin) | 7 endpoints | ✅ Completo |
| **Perfil Cliente** | Datos del cliente | 8 endpoints | ✅ Completo |
| **Productos** | Catálogo de productos | 11 endpoints | ✅ Completo |
| **Direcciones** | Direcciones de entrega | 9 endpoints | ✅ Completo |
| **Pedidos** | Gestión de pedidos | 6 endpoints | ✅ Completo |
| **Almacenes** | Ubicaciones de almacenes | 6 endpoints | ✅ Completo |

**Total: 48 endpoints funcionales**

### ⚠️ Pendientes para MVP+

| Módulo | Descripción | Prioridad |
|--------|-------------|-----------|
| **Transportistas** | Gestión de transportistas | Alta |
| **Zonas** | Asignación por zona geográfica | Alta |
| **WebSockets** | Tracking en tiempo real | Alta |
| **Notificaciones** | Email/SMS de estado | Media |
| **Facturas** | Generación de facturas | Media |
| **Pagos** | Procesamiento de pagos | Media |
| **Reportes** | Analytics y métricas | Baja |

## 4. Módulo de Autenticación y Registro

### Arquitectura Actual

```
┌─────────────────────────────────────────────────────────────┐
│                     FRONTEND (React/Vue)                    │
│                                                              │
│  1. Supabase Auth UI                                        │
│     - Registro (email/password)                             │
│     - Login                                                 │
│     - OAuth (Google, GitHub)                                │
│                                                              │
│  2. Guardado de Token                                        │
│     - localStorage: JWT access_token                        │
│     - localStorage: JWT refresh_token                       │
│                                                              │
│  3. Headers en Requests                                      │
│     Authorization: Bearer {access_token}                    │
└─────────────────┬───────────────────────────────────────────┘
                  │
                  │ HTTP + JWT
                  │
┌─────────────────▼───────────────────────────────────────────┐
│              BACKEND RUST (Axum + SQLx)                     │
│                                                              │
│  1. Middleware de Autenticación                             │
│     - Extrae Bearer token del header                        │
│     - Valida firma JWT con SUPABASE_JWT_SECRET              │
│     - Extrae claims (user_id, email, role)                 │
│                                                              │
│  2. Extension<AuthenticatedUser>                            │
│     - ID del usuario                                        │
│     - Email                                                 │
│     - Role                                                  │
│                                                              │
│  3. Handlers Protegidos                                      │
│     - Solo procesa requests con JWT válido                 │
│     - Rechaza con 401 si falta token                       │
│     - Rechaza con 401 si token expirado                    │
└─────────────────────────────────────────────────────────────┘
```

### Flujo de Autenticación en Detalle

#### 1️⃣ Registro (Frontend)

```javascript
// src/pages/Register.jsx
import { createClient } from '@supabase/supabase-js'

const supabase = createClient(VITE_SUPABASE_URL, VITE_SUPABASE_ANON_KEY)

async function handleRegister(email, password, nombre, telefono) {
  try {
    // Paso 1: Registrar en Supabase
    const { data, error } = await supabase.auth.signUp({
      email,
      password,
      options: {
        data: {
          nombre,
          telefono
        }
      }
    })

    if (error) throw error

    // Paso 2: Guardar token
    const token = data.session?.access_token
    localStorage.setItem('access_token', token)

    // Paso 3: Crear perfil en backend
    const response = await fetch('http://localhost:3000/api/perfil', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ nombre, telefono })
    })

    const profile = await response.json()
    return { success: true, user: data.user, profile }
  } catch (error) {
    console.error('Error en registro:', error)
  }
}
```

#### 2️⃣ Login (Frontend)

```javascript
// src/pages/Login.jsx
async function handleLogin(email, password) {
  const { data, error } = await supabase.auth.signInWithPassword({
    email,
    password
  })

  if (error) throw error

  // Guardar token
  const token = data.session.access_token
  localStorage.setItem('access_token', token)

  return { success: true, user: data.user }
}
```

#### 3️⃣ Request Autenticado (Frontend)

```javascript
// Cualquier fetch a endpoint protegido
const token = localStorage.getItem('access_token')

const response = await fetch('http://localhost:3000/api/perfil', {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${token}`
  }
})

const data = await response.json()
```

#### 4️⃣ Validación en Backend

```rust
// src/presentation/middleware/auth_middleware.rs

pub async fn require_auth(
    Request(mut req): Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extraer header Authorization
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized("Token missing".to_string()))?

    // Parsear "Bearer {token}"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid format".to_string()))?

    // Validar JWT con secret de Supabase
    let claims = validate_jwt(token, &SUPABASE_JWT_SECRET)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?

    // Verificar expiración
    if claims.is_expired() {
        return Err(AppError::Unauthorized("Token expired".to_string()))
    }

    // Convertir a AuthenticatedUser
    let user = AuthenticatedUser::from(claims)

    // Inyectar en Extension
    req.extensions_mut().insert(user)

    Ok(next.run(req).await)
}
```

#### 5️⃣ Acceso en Handlers

```rust
// src/presentation/handlers/pedido_handler.rs

#[utoipa::path(
    get,
    path = "/api/pedidos",
    security(("bearer_auth" = []))
)]
pub async fn list_pedidos(
    Extension(user): Extension<AuthenticatedUser>,  // ← Inyectado por middleware
    State(service): State<Arc<PedidoService>>,
) -> AppResult<Json<PedidosListResponseDTO>> {
    // user.id = UUID del usuario autenticado
    // user.email = email del usuario
    // user.role = rol en el sistema
    
    let pedidos = service.list_pedidos(user.id).await?
    Ok(Json(pedidos))
}
```

### Problemas Actuales y Soluciones

#### ❌ Problema 1: Registro Manual en Dos Pasos

El usuario debe registrarse en Supabase Y crear perfil manualmente en backend.

**Solución Propuesta:**
1. Implementar webhook de Supabase
2. Backend crea automáticamente perfil y usuario en `public.users`
3. O crear endpoint unificado de registro

#### ❌ Problema 2: Usuarios "Fantasma"

Usuario podría estar en Supabase pero no en `public.users`.

**Solución Propuesta:**
1. Middleware que verifica existencia de usuario antes de permitir acceso
2. Crear automáticamente si no existe
3. O requerir que admin cree usuario explícitamente

#### ❌ Problema 3: Roles sin Verificación

Backend confía en claims del JWT pero no verifica contra base de datos.

**Solución Propuesta:**
1. Verificar rol contra `public.users` en cada request
2. Implementar middleware de autorización por rol
3. Cache de permisos con TTL

### Mejoras Futuras para Login

```rust
// Middleware de autorización por rol (pendiente)
pub async fn require_admin(
    Extension(user): Extension<AuthenticatedUser>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Verificar que rol en BD es "admin"
    if user.role != Some("admin".to_string()) {
        return Err(AppError::Forbidden("Admin role required".to_string()))
    }
    
    Ok(next.run(req).await)
}

// Guards de permisos (pendiente)
pub async fn require_owner(
    user: AuthenticatedUser,
    id_recurso: Uuid,
) -> Result<(), AppError> {
    // Verificar que usuario es dueño del recurso
    let es_dueño = repository.verify_ownership(user.id, id_recurso).await?
    
    if !es_dueño {
        return Err(AppError::Forbidden("Not owner of resource".to_string()))
    }
    
    Ok(())
}
```

## 5. Ejemplo Completo: Crear un Pedido desde Cero

### Escenario

Juan se registra en la app y quiere comprar un producto.

### Pasos

#### 1. Juan se registra en frontend

```javascript
const response = await supabase.auth.signUp({
  email: 'juan@email.com',
  password: 'segura123',
  options: {
    data: {
      nombre: 'Juan',
      telefono: '+573001234567'
    }
  }
})

const token = response.session.access_token
localStorage.setItem('access_token', token)
```

#### 2. Admin crea usuario en BD (ACTUAL)

```bash
curl -X POST \
  -H "Authorization: Bearer {admin-token}" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "juan@email.com",
    "nombre": "Juan",
    "apellido": "",
    "rol": "cliente"
  }' \
  http://localhost:3000/api/admin/users
```

**O (FUTURO con webhook):**
Supabase dispara webhook → Backend crea automáticamente

#### 3. Juan crea perfil

```javascript
const perfil = await fetch('http://localhost:3000/api/perfil', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    nombre: 'Juan Pérez',
    telefono: '+573001234567'
  })
})
```

#### 4. Juan crea dirección

```javascript
const direccion = await fetch('http://localhost:3000/api/direcciones', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    calle: 'Calle 72 #15-45',
    ciudad: 'Bogotá',
    pais: 'Colombia',
    codigo_postal: '110111',
    latitud: 4.7110,
    longitud: -74.0721,
    es_predeterminada: true
  })
})
```

#### 5. Juan visualiza productos

```javascript
const productos = await fetch('http://localhost:3000/api/productos')
const data = await productos.json()
// Lista de productos disponibles
```

#### 6. Juan crea pedido

```javascript
const pedido = await fetch('http://localhost:3000/api/pedidos', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    id_producto: '550e8400-e29b-41d4-a716-446655440000',
    cantidad: 1,
    id_direccion_entrega: '660e8400-e29b-41d4-a716-446655440001',
    notas: 'Entregar en recepción'
  })
})
```

#### 7. Admin asigna transportista

```bash
curl -X PATCH \
  -H "Authorization: Bearer {admin-token}" \
  -H "Content-Type: application/json" \
  -d '{
    "id_transportista": "770e8400-e29b-41d4-a716-446655440002"
  }' \
  http://localhost:3000/api/pedidos/550e8400-e29b-41d4-a716-446655440000/transportista
```

#### 8. Transportista actualiza estado

```javascript
await fetch('http://localhost:3000/api/pedidos/{pedido_id}/estado', {
  method: 'PATCH',
  headers: {
    'Authorization': `Bearer ${transportista-token}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    estado: 'en_transito'
  })
})
```

## 6. Checklist de Implementación MVP

### Core Completado ✅
- [x] Auth con Supabase JWT
- [x] Usuarios (CRUD admin)
- [x] Perfiles de cliente
- [x] Productos (catálogo)
- [x] Pedidos (CRUD)
- [x] Direcciones
- [x] Almacenes

### Funcionalidades Pendientes ⚠️
- [ ] Transportistas (modelo, endpoints)
- [ ] Zonas geográficas
- [ ] Asignación inteligente de transportistas
- [ ] WebSockets para tracking
- [ ] Notificaciones
- [ ] Webhook de Supabase para sincronización
- [ ] Tests unitarios e integración
- [ ] Documentación Postman/OpenAPI

### Infraestructura ⚠️
- [ ] Docker compose para desarrollo
- [ ] Migraciones versionadas
- [ ] Variables de entorno correctas
- [ ] Base de datos sincronizada

## 7. Próximos Pasos

1. **Implementar Transportistas** (1-2 semanas)
   - Modelo y endpoints CRUD
   - Asignación de zonas
   - Estados de transportista

2. **WebSockets para Tracking** (1-2 semanas)
   - Conexión cliente-servidor
   - Actualización en tiempo real de ubicación
   - Notificaciones push

3. **Testing** (1 semana)
   - Tests unitarios de servicios
   - Tests de integración de endpoints
   - Coverage mínimo 80%

4. **Despliegue** (3-4 días)
   - Dockerfile y docker-compose
   - Variables de entorno en producción
   - CI/CD con GitHub Actions
