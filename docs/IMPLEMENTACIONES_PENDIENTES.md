# 📋 Implementaciones Pendientes

> **Última actualización**: 8 de diciembre de 2025

## Resumen de Estado Actual

### ✅ Completado

| Módulo | Repository | Repository Impl | Service | Handler | Routes | Swagger |
|--------|:----------:|:---------------:|:-------:|:-------:|:------:|:-------:|
| **Auth** | N/A | N/A | N/A | ✅ | ✅ | ✅ |
| **Pedidos** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

### ⬜ Pendiente

| Módulo | Repository | Repository Impl | Service | Handler | Routes | Swagger |
|--------|:----------:|:---------------:|:-------:|:-------:|:------:|:-------:|
| **Users** | ⬜ Trait | ⬜ Impl | ⬜ | ⬜ | ⬜ | ⬜ |
| **Productos** | ⬜ Trait | ⬜ Impl | ⬜ | ⬜ | ⬜ | ⬜ |
| **Direcciones** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Transportistas** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Zonas** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Facturas** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **Pagos** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **EventoPedidos** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| **PerfilesCliente** | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |

---

## Implementaciones Detalladas

### 1. 📦 CRUD Productos (Prioridad: ALTA)

**Archivos a modificar/crear:**

| Archivo | Estado | Descripción |
|---------|--------|-------------|
| `domain/repositories/producto_repository.rs` | 🔄 Incompleto | Agregar más métodos al trait |
| `infrastructure/repositories/producto_repository_impl.rs` | ⬜ Pendiente | Implementar queries SQLx |
| `application/dto/producto_dto.rs` | 🔄 Incompleto | Faltan más DTOs y conversiones |
| `application/services/producto_service.rs` | ⬜ Pendiente | Lógica de negocio |
| `presentation/handlers/producto_handler.rs` | ⬜ Pendiente | Endpoints con utoipa |
| `presentation/routes.rs` | 🔄 Pendiente | Agregar rutas de productos |

**Endpoints a implementar:**
```
GET    /api/productos              - Lista todos los productos
GET    /api/productos/{id}         - Obtiene un producto por ID
POST   /api/productos              - Crea un nuevo producto
PUT    /api/productos/{id}         - Actualiza un producto
DELETE /api/productos/{id}         - Elimina un producto (soft delete)
GET    /api/productos/categoria/{cat} - Filtra por categoría
GET    /api/productos/buscar?q=    - Búsqueda por nombre/SKU
PATCH  /api/productos/{id}/stock   - Actualizar stock
```

**Métodos adicionales para Repository:**
```rust
async fn find_by_sku(&self, sku: &str) -> Result<Option<Producto>, sqlx::Error>;
async fn find_by_categoria(&self, categoria: &str) -> Result<Vec<Producto>, sqlx::Error>;
async fn search(&self, query: &str) -> Result<Vec<Producto>, sqlx::Error>;
async fn update_stock(&self, id: Uuid, cantidad: i32) -> Result<Producto, sqlx::Error>;
async fn find_activos(&self) -> Result<Vec<Producto>, sqlx::Error>;
```

---

### 2. 👤 CRUD Users (Prioridad: ALTA)

**Nota:** Los usuarios se crean en `auth.users` (Supabase), esta tabla `public.users` es para datos extendidos.

**Archivos a modificar/crear:**

| Archivo | Estado | Descripción |
|---------|--------|-------------|
| `domain/repositories/user_repository.rs` | 🔄 Incompleto | Agregar sync con auth |
| `infrastructure/repositories/user_repository_impl.rs` | ⬜ Pendiente | Implementar queries |
| `application/dto/user_dto.rs` | 🔄 Existente | Verificar/actualizar |
| `application/services/user_service.rs` | ⬜ Pendiente | Lógica de negocio |
| `presentation/handlers/user_handler.rs` | ⬜ Pendiente | Endpoints |

**Endpoints a implementar:**
```
GET    /api/users                  - Lista todos los usuarios (admin)
GET    /api/users/{id}             - Obtiene un usuario por ID
PUT    /api/users/{id}             - Actualiza datos de usuario
GET    /api/users/me               - Obtiene el usuario actual (diferente a /auth/me)
PATCH  /api/users/{id}/rol         - Cambiar rol (admin only)
```

---

### 3. 📍 CRUD Direcciones (Prioridad: ALTA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/repositories/direccion_repository.rs` | Trait del repository |
| `infrastructure/repositories/direccion_repository_impl.rs` | Implementación SQLx |
| `application/dto/direccion_dto.rs` | DTOs |
| `application/services/direccion_service.rs` | Lógica de negocio |
| `presentation/handlers/direccion_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/direcciones                  - Direcciones del usuario actual
GET    /api/direcciones/{id}             - Obtiene una dirección
POST   /api/direcciones                  - Crea una dirección
PUT    /api/direcciones/{id}             - Actualiza una dirección
DELETE /api/direcciones/{id}             - Elimina una dirección
PATCH  /api/direcciones/{id}/predeterminada - Marcar como predeterminada
```

**Consideraciones especiales:**
- Manejo de PostGIS para `ubicacion_geo`
- Solo el dueño puede ver/editar sus direcciones

---

### 4. 🚚 CRUD Transportistas (Prioridad: MEDIA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/repositories/transportista_repository.rs` | Trait del repository |
| `infrastructure/repositories/transportista_repository_impl.rs` | Implementación SQLx |
| `application/dto/transportista_dto.rs` | DTOs |
| `application/services/transportista_service.rs` | Lógica de negocio |
| `presentation/handlers/transportista_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/transportistas               - Lista transportistas
GET    /api/transportistas/{id}          - Obtiene un transportista
POST   /api/transportistas               - Registra nuevo transportista
PUT    /api/transportistas/{id}          - Actualiza datos
PATCH  /api/transportistas/{id}/estado   - Cambiar estado (activar/suspender)
GET    /api/transportistas/zona/{id}     - Por zona asignada
GET    /api/transportistas/disponibles   - Transportistas activos sin pedido
```

---

### 5. 🗺️ CRUD Zonas (Prioridad: MEDIA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/repositories/zona_repository.rs` | Trait |
| `infrastructure/repositories/zona_repository_impl.rs` | Implementación |
| `application/dto/zona_dto.rs` | DTOs |
| `application/services/zona_service.rs` | Lógica |
| `presentation/handlers/zona_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/zonas                  - Lista zonas
GET    /api/zonas/{id}             - Obtiene una zona
POST   /api/zonas                  - Crea zona (admin)
PUT    /api/zonas/{id}             - Actualiza zona
DELETE /api/zonas/{id}             - Desactiva zona
GET    /api/zonas/ciudad/{ciudad}  - Zonas por ciudad
```

---

### 6. 🧾 CRUD Facturas (Prioridad: MEDIA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/repositories/factura_repository.rs` | Trait |
| `infrastructure/repositories/factura_repository_impl.rs` | Implementación |
| `application/dto/factura_dto.rs` | DTOs |
| `application/services/factura_service.rs` | Lógica |
| `presentation/handlers/factura_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/facturas                   - Facturas del usuario
GET    /api/facturas/{id}              - Obtiene una factura
POST   /api/facturas                   - Genera factura para pedido
GET    /api/facturas/pedido/{id}       - Factura de un pedido
PATCH  /api/facturas/{id}/estado       - Cambiar estado
GET    /api/facturas/{id}/pdf          - Descargar PDF (futuro)
```

---

### 7. 💳 CRUD Pagos (Prioridad: MEDIA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/repositories/pago_repository.rs` | Trait |
| `infrastructure/repositories/pago_repository_impl.rs` | Implementación |
| `application/dto/pago_dto.rs` | DTOs |
| `application/services/pago_service.rs` | Lógica |
| `presentation/handlers/pago_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/pagos                      - Pagos del usuario
GET    /api/pagos/{id}                 - Obtiene un pago
POST   /api/pagos                      - Registra un pago
GET    /api/pagos/factura/{id}         - Pagos de una factura
PATCH  /api/pagos/{id}/estado          - Actualizar estado
```

---

### 8. 📜 Eventos de Pedidos (Prioridad: BAJA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/entities/evento_pedido.rs` | Entidad |
| `domain/repositories/evento_pedido_repository.rs` | Trait |
| `infrastructure/repositories/evento_pedido_repository_impl.rs` | Implementación |
| `application/services/evento_pedido_service.rs` | Lógica |
| `presentation/handlers/evento_pedido_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/pedidos/{id}/eventos       - Historial de eventos de un pedido
POST   /api/pedidos/{id}/eventos       - Registrar evento (interno)
```

---

### 9. 👥 Perfiles Cliente (Prioridad: ALTA)

**Archivos a crear:**

| Archivo | Descripción |
|---------|-------------|
| `domain/entities/perfil_cliente.rs` | Entidad |
| `domain/repositories/perfil_cliente_repository.rs` | Trait |
| `infrastructure/repositories/perfil_cliente_repository_impl.rs` | Implementación |
| `application/dto/perfil_cliente_dto.rs` | DTOs |
| `application/services/perfil_cliente_service.rs` | Lógica |
| `presentation/handlers/perfil_cliente_handler.rs` | Endpoints |

**Endpoints a implementar:**
```
GET    /api/perfil                     - Perfil del usuario actual
POST   /api/perfil                     - Crear perfil de cliente
PUT    /api/perfil                     - Actualizar perfil
```

---

## Funcionalidades Adicionales Pendientes

### 🔌 WebSockets / Tiempo Real (Prioridad: ALTA)
- Tracking en vivo de pedidos
- Notificaciones push
- Actualizaciones de estado

### 🔐 Autorización por Roles (Prioridad: ALTA)
- Middleware para verificar roles
- Guards para endpoints admin
- Restricciones por propietario

### 📊 Reportes y Analytics (Prioridad: BAJA)
- Dashboard de métricas
- Reportes de ventas
- Estadísticas de entregas

### 🔍 Búsqueda Avanzada (Prioridad: BAJA)
- Full-text search en productos
- Filtros combinados
- Paginación cursor-based

---

## Orden de Implementación Recomendado

1. **Perfiles Cliente** - Base para direcciones y pedidos
2. **Productos** - CRUD completo, necesario para pedidos
3. **Direcciones** - Necesario para crear pedidos
4. **Users** - Gestión de usuarios
5. **Transportistas** - Asignación a pedidos
6. **Zonas** - Configuración geográfica
7. **Facturas** - Facturación
8. **Pagos** - Procesamiento de pagos
9. **Eventos Pedidos** - Auditoría
10. **WebSockets** - Tiempo real

---

## Notas Técnicas

### Patrón a seguir (basado en Pedidos)

```
1. domain/repositories/{entity}_repository.rs     # Trait abstracto
2. infrastructure/repositories/{entity}_repository_impl.rs  # SQLx impl
3. application/dto/{entity}_dto.rs                 # DTOs + From conversions
4. application/services/{entity}_service.rs        # Business logic
5. presentation/handlers/{entity}_handler.rs       # Handlers + utoipa
6. presentation/routes.rs                          # Registrar rutas
```

### Checklist por módulo:
- [ ] Entidad existe en `domain/entities/`
- [ ] Repository trait definido
- [ ] Repository implementado con SQLx
- [ ] DTOs con ToSchema y serde
- [ ] Service con lógica de negocio
- [ ] Handlers con utoipa anotaciones
- [ ] Rutas registradas
- [ ] Swagger paths agregados
- [ ] Tests unitarios (futuro)
