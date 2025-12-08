# 📦 Módulo de Pedidos

## Arquitectura (SOLID + Clean Architecture)

```
┌─────────────────────────────────────────────────────────────────┐
│                        Presentation                              │
│  pedido_handler.rs (HTTP handlers - thin layer)                  │
├─────────────────────────────────────────────────────────────────┤
│                        Application                               │
│  pedido_service.rs (business logic, orchestration)               │
│  pedido_dto.rs (request/response DTOs)                           │
├─────────────────────────────────────────────────────────────────┤
│                         Domain                                   │
│  pedido.rs (entity)                                              │
│  pedido_repository.rs (trait - interface)                        │
├─────────────────────────────────────────────────────────────────┤
│                      Infrastructure                              │
│  pedido_repository_impl.rs (SQLx implementation)                 │
└─────────────────────────────────────────────────────────────────┘
```

## Principios Aplicados

### SRP (Single Responsibility)
- **Handler**: Solo recibe HTTP, valida input, delega al service
- **Service**: Lógica de negocio, validaciones de dominio
- **Repository**: Solo acceso a datos

### OCP (Open/Closed)
- Nuevos estados de pedido se agregan sin modificar código existente
- Repository trait permite cambiar implementación (testing, otra DB)

### LSP (Liskov Substitution)
- Cualquier implementación de `PedidoRepository` es intercambiable

### ISP (Interface Segregation)
- Traits específicos por operación si es necesario

### DIP (Dependency Inversion)
- Service depende del trait, no de la implementación
- Inyección de dependencias via constructor

## Estados del Pedido

```
pendiente → confirmado → en_transito → entregado
     │           │             │
     └───────────┴─────────────┴──→ cancelado
```

## Endpoints

| Método | Ruta | Auth | Descripción |
|--------|------|------|-------------|
| GET | `/api/pedidos` | ✅ | Listar pedidos del usuario |
| GET | `/api/pedidos/:id` | ✅ | Obtener pedido por ID |
| POST | `/api/pedidos` | ✅ | Crear nuevo pedido |
| PATCH | `/api/pedidos/:id/estado` | ✅ | Actualizar estado |
| DELETE | `/api/pedidos/:id` | ✅ | Cancelar pedido |

## Flujo de Creación

```
1. Cliente envía CreatePedidoDTO
2. Handler valida estructura
3. Service valida reglas de negocio:
   - Usuario existe
   - Direcciones existen
   - Stock disponible (si aplica)
4. Repository persiste el pedido
5. Se genera numero_tracking automático
6. Retorna PedidoResponseDTO
```

## Manejo de Errores

Errores centralizados en `AppError`:
- `NotFound` → 404
- `BadRequest` → 400
- `Unauthorized` → 401
- `Forbidden` → 403
- `Internal` → 500
