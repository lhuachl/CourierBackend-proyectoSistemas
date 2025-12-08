# CRUD Direcciones

## Descripción

Módulo de gestión de direcciones de clientes y almacenes. Soporta soft delete por defecto y hard delete opcional.

## Tipos de Dirección

- **cliente**: Direcciones asociadas a un perfil de cliente
- **almacen**: Direcciones de almacenes (sin perfil asociado)

## Endpoints

### Cliente (Protegidos con JWT)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/api/direcciones` | Listar mis direcciones |
| GET | `/api/direcciones/predeterminada` | Obtener dirección predeterminada |
| GET | `/api/direcciones/{id}` | Obtener dirección por ID |
| POST | `/api/direcciones` | Crear nueva dirección |
| PUT | `/api/direcciones/{id}` | Actualizar dirección |
| PATCH | `/api/direcciones/{id}/predeterminada` | Establecer como predeterminada |
| DELETE | `/api/direcciones/{id}` | Desactivar dirección (soft delete) |
| PATCH | `/api/direcciones/{id}/activar` | Reactivar dirección |
| DELETE | `/api/direcciones/{id}/permanente` | Eliminar permanentemente (hard delete) |

### Almacenes (Públicos)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/api/almacenes` | Listar almacenes activos |

### Admin Almacenes (Protegidos)

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/api/admin/almacenes` | Listar todos los almacenes (incluye inactivos) |
| POST | `/api/admin/almacenes` | Crear nuevo almacén |
| DELETE | `/api/admin/almacenes/{id}` | Desactivar almacén (soft delete) |
| PATCH | `/api/admin/almacenes/{id}/activar` | Reactivar almacén |
| DELETE | `/api/admin/almacenes/{id}/permanente` | Eliminar permanentemente |

## DTOs

### CreateDireccionDTO (Cliente)

```json
{
  "etiqueta": "Casa",             // String
  "direccion_linea1": "Av. Principal 123",  // String
  "direccion_linea2": null,       // Optional<String>
  "ciudad": "Lima",               // String
  "region": "Lima",               // String
  "codigo_postal": "15001",       // Optional<String>
  "pais": "PE",                   // String (default: "PE")
  "latitud": -12.0464,            // Optional<Decimal>
  "longitud": -77.0428,           // Optional<Decimal>
  "instrucciones_entrega": null,  // Optional<String>
  "telefono_contacto": null,      // Optional<String>
  "es_predeterminada": false      // bool (default: false)
}
```

### CreateAlmacenDTO (Admin)

```json
{
  "etiqueta": "Almacén Central",       // String
  "direccion_linea1": "Zona Industrial", // String
  "direccion_linea2": null,            // Optional<String>
  "ciudad": "Lima",                    // String
  "region": "Lima",                    // String
  "codigo_postal": "15001",            // Optional<String>
  "pais": "PE",                        // String (default: "PE")
  "latitud": -12.0464,                 // Optional<Decimal>
  "longitud": -77.0428,                // Optional<Decimal>
  "instrucciones_entrega": null,       // Optional<String>
  "telefono_contacto": null            // Optional<String>
}
```

### UpdateDireccionDTO

```json
{
  "etiqueta": "Oficina",          // Optional<String>
  "direccion_linea1": null,       // Optional<String>
  "direccion_linea2": null,       // Optional<String>
  "ciudad": null,                 // Optional<String>
  "region": null,                 // Optional<String>
  "codigo_postal": null,          // Optional<String>
  "pais": null,                   // Optional<String>
  "latitud": null,                // Optional<Decimal>
  "longitud": null,               // Optional<Decimal>
  "instrucciones_entrega": null,  // Optional<String>
  "telefono_contacto": null       // Optional<String>
}
```

### DireccionResponseDTO

```json
{
  "id_direccion": "uuid",
  "id_perfil": "uuid",            // null para almacenes
  "tipo": "cliente",              // "cliente" | "almacen"
  "etiqueta": "Casa",
  "direccion_linea1": "Av. Principal 123",
  "direccion_linea2": null,
  "ciudad": "Lima",
  "region": "Lima",
  "codigo_postal": "15001",
  "pais": "PE",
  "latitud": -12.0464,
  "longitud": -77.0428,
  "instrucciones_entrega": null,
  "telefono_contacto": null,
  "es_predeterminada": true,
  "activo": true,
  "fecha_creacion": "2024-01-01T00:00:00Z",
  "fecha_actualizacion": "2024-01-01T00:00:00Z"
}
```

## Reglas de Negocio

### Límites
- Máximo **10 direcciones** por perfil de cliente
- Solo se puede tener **1 dirección predeterminada** activa

### Validaciones de Coordenadas
- Latitud: -90.0 a 90.0
- Longitud: -180.0 a 180.0

### Soft Delete vs Hard Delete
- **Soft delete** (recomendado): Desactiva la dirección (`activo = false`)
  - La dirección se mantiene en la base de datos
  - Puede ser reactivada posteriormente
  - Útil para mantener historial de entregas
  
- **Hard delete**: Elimina permanentemente la dirección
  - Acción irreversible
  - Usar solo cuando sea necesario por temas de privacidad

### Dirección Predeterminada
- Al establecer una dirección como predeterminada, las demás pierden ese estado
- No se puede establecer como predeterminada una dirección inactiva
- Si se desactiva la dirección predeterminada, el usuario queda sin predeterminada

### Almacenes
- Los almacenes NO tienen `id_perfil` (es null)
- Los almacenes NO pueden tener `es_predeterminada = true`
- Solo administradores pueden gestionar almacenes
- Los almacenes son visibles públicamente (solo activos)

## Arquitectura

```
src/
├── domain/
│   ├── entities/direccion.rs         # Entidad Direccion + TipoDireccion
│   └── repositories/direccion_repository.rs  # Trait del repositorio
├── infrastructure/
│   └── repositories/direccion_repository_impl.rs  # Implementación SQLx
├── application/
│   ├── dto/direccion_dto.rs          # DTOs con validaciones
│   └── services/direccion_service.rs # Lógica de negocio
└── presentation/
    ├── handlers/direccion_handler.rs # Handlers HTTP
    └── routes.rs                     # Definición de rutas
```

## Ejemplos de Uso

### Crear Dirección

```bash
curl -X POST http://localhost:3000/api/direcciones \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "etiqueta": "Casa",
    "direccion_linea1": "Av. Javier Prado 123",
    "ciudad": "Lima",
    "region": "Lima",
    "pais": "PE",
    "latitud": -12.0900,
    "longitud": -77.0500,
    "es_predeterminada": true
  }'
```

### Listar Almacenes

```bash
curl http://localhost:3000/api/almacenes
```

### Desactivar Dirección (Soft Delete)

```bash
curl -X DELETE http://localhost:3000/api/direcciones/<id> \
  -H "Authorization: Bearer <token>"
```

### Eliminar Permanentemente (Hard Delete)

```bash
curl -X DELETE http://localhost:3000/api/direcciones/<id>/permanente \
  -H "Authorization: Bearer <token>"
```

## Notas de Implementación

- El campo `ubicacion_geo` es manejado por PostGIS en la base de datos y no se expone en la API
- Las coordenadas se validan pero no se requieren para crear una dirección
- El campo `tipo` se establece automáticamente según el endpoint utilizado
