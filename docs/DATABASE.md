# 🗄️ Base de Datos

## Conexión

El proyecto se conecta a PostgreSQL via Supabase usando SQLx con pool de conexiones.

## Diagrama de Relaciones

```
┌─────────────────┐     ┌─────────────────────┐     ┌─────────────────┐
│   auth.users    │     │       users         │     │ perfiles_cliente│
│   (Supabase)    │◄────│  (datos extendidos) │◄────│   (clientes)    │
└─────────────────┘     └─────────────────────┘     └────────┬────────┘
                                   │                         │
                                   │                         │
                        ┌──────────┴──────────┐              │
                        ▼                     ▼              ▼
               ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
               │  transportistas │   │    direcciones  │   │     pedidos     │
               └────────┬────────┘   └────────┬────────┘   └────────┬────────┘
                        │                     │                     │
                        │                     └─────────────────────┤
                        │                                           │
               ┌────────┴────────┐                         ┌────────┴────────┐
               │      zonas      │                         │ evento_pedidos  │
               └─────────────────┘                         └─────────────────┘
                                                                    │
                                                           ┌────────┴────────┐
                                                           │    facturas     │
                                                           └────────┬────────┘
                                                                    │
                                                           ┌────────┴────────┐
                                                           │      pagos      │
                                                           └─────────────────┘
```

## Tablas y Relaciones

### Core - Usuarios y Perfiles

| Tabla | Descripción | FK |
|-------|-------------|-----|
| `auth.users` | Tabla de Supabase Auth (externa) | - |
| `users` | Datos extendidos del usuario | → `auth.users(id)` |
| `perfiles_cliente` | Perfil de cliente con datos adicionales | → `users(id)` |

### Logística - Pedidos y Entregas

| Tabla | Descripción | FK |
|-------|-------------|-----|
| `pedidos` | Órdenes de envío | → `perfiles_cliente`, `transportistas`, `direcciones` (x2), `productos` |
| `direcciones` | Direcciones de origen/destino | → `perfiles_cliente` |
| `transportistas` | Datos de transportistas | → `users`, `zonas` |
| `zonas` | Zonas geográficas de cobertura | - |
| `evento_pedidos` | Auditoría de cambios de estado | → `pedidos`, `users` |

### Facturación y Pagos

| Tabla | Descripción | FK |
|-------|-------------|-----|
| `facturas` | Facturas emitidas | → `pedidos` |
| `pagos` | Registros de pagos | → `facturas` |

### Catálogo

| Tabla | Descripción | FK |
|-------|-------------|-----|
| `productos` | Catálogo de productos | - |

## Tipos ENUM

```sql
-- Roles de usuario
CREATE TYPE rol_usuario AS ENUM ('cliente', 'transportista', 'admin');

-- Estados de pedido
CREATE TYPE estado_pedido AS ENUM (
  'pendiente', 'confirmado', 'en_transito', 'entregado', 'cancelado'
);

-- Estados de factura
CREATE TYPE estado_factura AS ENUM ('pendiente', 'pagada', 'vencida', 'cancelada');

-- Estados de pago
CREATE TYPE estado_pago AS ENUM ('pendiente', 'completado', 'fallido', 'reembolsado');

-- Métodos de pago
CREATE TYPE metodo_pago_enum AS ENUM (
  'tarjeta_credito', 'tarjeta_debito', 'transferencia', 'efectivo', 'billetera_digital'
);

-- Estados de transportista
CREATE TYPE estado_transportista AS ENUM (
  'verificacion_pendiente', 'activo', 'inactivo', 'suspendido'
);
```

## Campos Especiales

### PostGIS (Geolocalización)
- `direcciones.ubicacion_geo` - Punto geográfico (latitud, longitud)
- `zonas.poligono_geo` - Polígono de zona de cobertura

### Generados Automáticamente
- `pedidos.numero_tracking` - `TRK-XXXXXXXX` (auto-generado)
- `facturas.total` - Calculado: `subtotal + impuestos`

## Problemas Identificados

### 1. Diseño Desnormalizado en `pedidos`
La tabla `pedidos` tiene campos de producto (`id_producto`, `cantidad`, `precio_unitario`) que deberían estar en una tabla `items_pedido` separada.

**Recomendación**: Crear tabla `items_pedido` para relación N:M.

### 2. Campos Duplicados en `users`
- `nombre`/`apellido` vs `name`
- `Gmail` debería ser `email`

### 3. Campo `Ubicacion` Redundante
En `pedidos` existe un campo `Ubicacion` (con mayúscula) que parece duplicar `id_direccion_destino`.

## Índices Recomendados

```sql
-- Performance en consultas frecuentes
CREATE INDEX idx_pedidos_estado ON pedidos(estado);
CREATE INDEX idx_pedidos_perfil ON pedidos(id_perfil);
CREATE INDEX idx_pedidos_transportista ON pedidos(id_transportista);
CREATE INDEX idx_productos_categoria ON productos(categoria);
CREATE INDEX idx_productos_sku ON productos(sku);
CREATE INDEX idx_direcciones_perfil ON direcciones(id_perfil);
```

## Migraciones

### Crear nueva migración
```bash
sqlx migrate add -r nombre_migracion
```

### Ejecutar migraciones
```bash
sqlx migrate run
```

### Revertir última migración
```bash
sqlx migrate revert
```
