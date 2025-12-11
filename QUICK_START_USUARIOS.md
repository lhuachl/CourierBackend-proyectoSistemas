# 🎯 Quick Start - Módulo de Usuarios

## Archivos Clave

```
src/
├── domain/
│   ├── entities/user.rs              ✅ Entidad User
│   └── repositories/user_repository.rs ✅ Trait con 8 métodos
├── infrastructure/
│   └── repositories/user_repository_impl.rs ✅ Implementación SQLx
├── application/
│   ├── dto/user_dto.rs               ✅ 6 DTOs
│   └── services/user_service.rs      ✅ 9 métodos de lógica
└── presentation/
    ├── handlers/user_handler.rs      ✅ 7 handlers
    └── routes.rs                     ✅ Rutas integradas

docs/
├── USUARIOS.md                       ✅ Documentación técnica
├── MVP_REQUERIMIENTOS.md             ✅ Requisitos y flujos
└── README.md                         ✅ Actualizado

migrations/
└── 002_add_user_fields.sql           ✅ Migración BD
```

## 7 Endpoints Creados

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/admin/users` | Listar usuarios |
| GET | `/api/admin/users/{id}` | Obtener usuario |
| POST | `/api/admin/users` | Crear usuario |
| PUT | `/api/admin/users/{id}` | Actualizar usuario |
| PATCH | `/api/admin/users/{id}/role` | **NEW** Cambiar rol |
| PATCH | `/api/admin/users/{id}/status` | **NEW** Cambiar estado |
| DELETE | `/api/admin/users/{id}` | Eliminar (soft delete) |

## ⚡ Usar Rápido

### 1. Crear Usuario

```bash
curl -X POST \
  -H "Authorization: Bearer {jwt}" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "juan@example.com",
    "nombre": "Juan",
    "apellido": "Pérez",
    "rol": "cliente"
  }' \
  http://localhost:3000/api/admin/users
```

### 2. Listar Usuarios

```bash
curl -H "Authorization: Bearer {jwt}" \
  http://localhost:3000/api/admin/users
```

### 3. Cambiar Rol a Admin

```bash
curl -X PATCH \
  -H "Authorization: Bearer {jwt}" \
  -H "Content-Type: application/json" \
  -d '{"rol": "admin"}' \
  http://localhost:3000/api/admin/users/{user_id}/role
```

### 4. Suspender Usuario

```bash
curl -X PATCH \
  -H "Authorization: Bearer {jwt}" \
  -H "Content-Type: application/json" \
  -d '{"activo": false}' \
  http://localhost:3000/api/admin/users/{user_id}/status
```

## 📚 Documentación

- **docs/USUARIOS.md** - Todo sobre el módulo de usuarios
- **docs/MVP_REQUERIMIENTOS.md** - Flujos completos, roles, ejemplo de registro  
- **Swagger UI** - http://localhost:3000/swagger-ui (tab "usuarios-admin")

## 🏗️ Arquitectura

```
Request HTTP
    ↓
require_auth (middleware)
    ↓
UserHandler (extrae parámetros)
    ↓
UserService (lógica de negocio)
    ↓
UserRepository (abstracción)
    ↓
UserRepositoryImpl (SQLx queries)
    ↓
PostgreSQL
```

## 🔐 Requisitos de Acceso

- ✅ Token JWT válido (header `Authorization: Bearer {token}`)
- ✅ Usuario autenticado en Supabase
- ✅ Rol "admin" (verificar en endpoint `/auth/me`)
- ✅ SUPABASE_JWT_SECRET configurada

## 📋 DTOs Disponibles

```rust
// Crear
CreateUserDTO {
  email: String,
  nombre: Option<String>,
  apellido: Option<String>,
  rol: String = "cliente"
}

// Actualizar
UpdateUserDTO {
  email: Option<String>,
  nombre: Option<String>,
  apellido: Option<String>,
  rol: Option<String>,
  foto_perfil: Option<String>
}

// Cambiar rol
UpdateUserRoleDTO {
  rol: String
}

// Cambiar estado
UpdateUserStatusDTO {
  activo: bool
}

// Respuesta
UserResponseDTO {
  id: Uuid,
  email: Option<String>,
  nombre: Option<String>,
  apellido: Option<String>,
  rol: String,
  foto_perfil: Option<String>,
  activo: bool,
  created_at: DateTime,
  updated_at: DateTime
}
```

## ✅ Estado

- ✅ Código compilando sin errores
- ✅ 7 endpoints funcionales
- ✅ Documentación completa (500+ líneas)
- ✅ DTOs con validación
- ✅ Migración SQL incluida
- ⚠️ Tests unitarios (pendiente)
- ⚠️ Middleware de rol (pendiente)

## 🚀 Próximo

Ver `docs/MVP_REQUERIMIENTOS.md` para:
- Flujo completo de registro de usuario
- Cómo crear pedido desde cero
- Arquitectura de autenticación
- Roadmap del MVP

---

**¿Necesitas ayuda?**  
Consulta `docs/USUARIOS.md` o `docs/MVP_REQUERIMIENTOS.md`
