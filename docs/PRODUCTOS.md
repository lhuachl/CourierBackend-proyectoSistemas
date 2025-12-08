# 📦 Módulo Productos

## Descripción

El módulo de productos gestiona el catálogo completo de productos disponibles para pedidos. Incluye endpoints públicos para consulta y endpoints administrativos para gestión.

## Arquitectura

```
┌─────────────────────────────────────────────────────────────────┐
│                    PRESENTATION LAYER                           │
│  producto_handler.rs                                            │
│  - list_productos, get_producto, search_productos               │
│  - get_by_categoria, get_by_sku                                 │
│  - create_producto, update_producto, update_stock               │
│  - update_estado_producto, delete_producto                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    APPLICATION LAYER                            │
│  ProductoService                                                │
│  - Validación de negocio (precio > 0, stock >= 0)               │
│  - Validación de SKU único y formato                            │
│  - Conversión Decimal ↔ f64                                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DOMAIN LAYER                               │
│  ProductoRepository (trait)                                     │
│  - find_by_id, find_by_sku, find_activos, find_all              │
│  - find_by_categoria, search                                    │
│  - create, update, update_stock, update_estado, delete          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  INFRASTRUCTURE LAYER                           │
│  ProductoRepositoryImpl                                         │
│  - Queries SQLx a PostgreSQL                                    │
│  - Manejo de tipos Decimal                                      │
└─────────────────────────────────────────────────────────────────┘
```

## Endpoints

### Públicos (Sin autenticación)

| Método | Ruta | Descripción |
|--------|------|-------------|
| `GET` | `/api/productos` | Lista productos activos |
| `GET` | `/api/productos/{id}` | Obtiene producto por ID |
| `GET` | `/api/productos/buscar?q=` | Búsqueda por nombre/SKU/descripción |
| `GET` | `/api/productos/categoria/{cat}` | Filtra por categoría |
| `GET` | `/api/productos/sku/{sku}` | Obtiene producto por SKU |

### Administrativos (Requieren autenticación)

| Método | Ruta | Descripción |
|--------|------|-------------|
| `GET` | `/api/admin/productos` | Lista todos (incluye inactivos) |
| `POST` | `/api/admin/productos` | Crea un producto |
| `PUT` | `/api/admin/productos/{id}` | Actualiza un producto |
| `PATCH` | `/api/admin/productos/{id}/stock` | Ajusta stock (+/-) |
| `PATCH` | `/api/admin/productos/{id}/estado` | Activa/desactiva |
| `DELETE` | `/api/admin/productos/{id}` | Elimina (hard delete) |

## DTOs

### CreateProductoDTO (Request)

```json
{
  "nombre_producto": "Laptop Dell XPS 15",
  "descripcion": "Laptop de alta gama con procesador Intel i7",
  "precio": 1299.99,
  "stock": 50,
  "categoria": "Electrónicos",
  "sku": "DELL-XPS15-2024"
}
```

### UpdateProductoDTO (Request)

```json
{
  "nombre_producto": "Laptop Dell XPS 15 Pro",
  "descripcion": "Nueva descripción",
  "precio": 1499.99,
  "categoria": "Laptops",
  "sku": "DELL-XPS15PRO-2024"
}
```

### UpdateStockDTO (Request)

```json
{
  "cantidad": 10,
  "motivo": "Recepción de inventario"
}
```

> **Nota**: `cantidad` puede ser positiva (agregar) o negativa (restar)

### UpdateEstadoProductoDTO (Request)

```json
{
  "estado": false
}
```

### ProductoResponseDTO (Response)

```json
{
  "id_producto": "550e8400-e29b-41d4-a716-446655440000",
  "nombre_producto": "Laptop Dell XPS 15",
  "descripcion": "Laptop de alta gama",
  "precio": 1299.99,
  "stock": 50,
  "categoria": "Electrónicos",
  "sku": "DELL-XPS15-2024",
  "estado": true,
  "created_at": "2024-12-08T10:00:00Z",
  "updated_at": "2024-12-08T10:00:00Z"
}
```

## Validaciones de Negocio

### Creación

| Campo | Validación |
|-------|------------|
| `nombre_producto` | Requerido, no puede estar vacío |
| `precio` | Requerido, debe ser > 0 |
| `stock` | Opcional (default: 0), debe ser >= 0 |
| `sku` | Opcional, debe ser único, solo alfanumérico + guiones |

### Actualización de Stock

- El stock resultante no puede ser negativo
- Se registra el motivo del ajuste (logging)

### SKU

- Formato: Solo letras, números, guiones (`-`) y underscores (`_`)
- Longitud máxima: 50 caracteres
- Debe ser único en toda la base de datos

## Modelo de Datos

### Entidad Producto

```rust
pub struct Producto {
    pub id_producto: Uuid,
    pub nombre_producto: String,
    pub descripcion: Option<String>,
    pub precio: Decimal,          // rust_decimal para precisión
    pub stock: i32,
    pub categoria: Option<String>,
    pub sku: Option<String>,
    pub estado: bool,             // true = activo
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Tabla PostgreSQL

```sql
CREATE TABLE productos (
  id_producto uuid NOT NULL DEFAULT uuid_generate_v4(),
  nombre_producto varchar NOT NULL,
  descripcion text,
  precio numeric NOT NULL CHECK (precio > 0),
  stock integer NOT NULL DEFAULT 0 CHECK (stock >= 0),
  categoria varchar,
  sku varchar UNIQUE,
  estado boolean NOT NULL DEFAULT true,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (id_producto)
);
```

## Ejemplos de Uso

### Crear producto

```bash
curl -X POST http://localhost:3000/api/admin/productos \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "nombre_producto": "Teclado Mecánico",
    "precio": 89.99,
    "stock": 100,
    "categoria": "Periféricos",
    "sku": "TEC-MEC-001"
  }'
```

### Buscar productos

```bash
curl "http://localhost:3000/api/productos/buscar?q=laptop"
```

### Ajustar stock

```bash
# Agregar 20 unidades
curl -X PATCH http://localhost:3000/api/admin/productos/{id}/stock \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"cantidad": 20, "motivo": "Compra proveedor"}'

# Restar 5 unidades
curl -X PATCH http://localhost:3000/api/admin/productos/{id}/stock \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"cantidad": -5, "motivo": "Ajuste inventario"}'
```

### Desactivar producto

```bash
curl -X PATCH http://localhost:3000/api/admin/productos/{id}/estado \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"estado": false}'
```

## Consideraciones

### Soft Delete vs Hard Delete

Actualmente se usa **hard delete**. Para producción, se recomienda implementar soft delete:

```rust
// Cambiar delete por:
async fn soft_delete(&self, id: Uuid) -> AppResult<()> {
    sqlx::query("UPDATE productos SET estado = false, deleted_at = NOW() WHERE id_producto = $1")
        .bind(id)
        .execute(&self.pool)
        .await?;
    Ok(())
}
```

### Precisión de Precios

Se usa `rust_decimal::Decimal` para evitar errores de punto flotante en operaciones financieras. La conversión a `f64` solo se hace en los DTOs de respuesta.

### Índices Recomendados

```sql
CREATE INDEX idx_productos_categoria ON productos(categoria);
CREATE INDEX idx_productos_sku ON productos(sku);
CREATE INDEX idx_productos_estado ON productos(estado);
CREATE INDEX idx_productos_nombre ON productos USING gin(to_tsvector('spanish', nombre_producto));
```

## Próximas Mejoras

- [ ] Paginación cursor-based
- [ ] Imágenes de producto
- [ ] Variantes de producto (tallas, colores)
- [ ] Historial de precios
- [ ] Alertas de stock bajo
- [ ] Categorías como entidad separada
