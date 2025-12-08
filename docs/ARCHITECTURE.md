# 🏗️ Arquitectura del Sistema

## Clean Architecture

El proyecto sigue Clean Architecture con 4 capas concéntricas:

```
┌─────────────────────────────────────────────────────────┐
│                    Presentation                          │
│  (handlers, routes, middleware, WebSockets)              │
├─────────────────────────────────────────────────────────┤
│                    Application                           │
│  (services, DTOs, casos de uso)                          │
├─────────────────────────────────────────────────────────┤
│                      Domain                              │
│  (entities, repository traits, reglas de negocio)        │
├─────────────────────────────────────────────────────────┤
│                   Infrastructure                         │
│  (implementaciones de repos, BD, servicios externos)     │
└─────────────────────────────────────────────────────────┘
```

## Regla de Dependencias

Las dependencias SIEMPRE apuntan hacia adentro:
- `Presentation` → `Application` → `Domain`
- `Infrastructure` implementa interfaces de `Domain`
- `Domain` NO depende de ninguna otra capa

## Flujo de una Request

```
HTTP Request
    ↓
[Middleware] → Autenticación, logging, CORS
    ↓
[Handler] → Extrae datos, valida input
    ↓
[Service] → Lógica de aplicación
    ↓
[Repository Trait] → Interfaz (Domain)
    ↓
[Repository Impl] → Consulta real (Infrastructure)
    ↓
[Database] → PostgreSQL/Supabase
```

## Decisiones de Diseño

### ¿Por qué Axum?
- Built on Tokio (ecosistema unificado)
- Extractors type-safe
- Compatible con Tower middleware
- Performance excelente

### ¿Por qué SQLx sobre Diesel/SeaORM?
- Validación en tiempo de compilación
- SQL puro cuando se necesita
- Async nativo
- Sin ORM overhead

### ¿Por qué traits para repositorios?
- Inyección de dependencias
- Testing con mocks
- Cambio de implementación sin afectar dominio

## Manejo de Errores

Se usa un tipo `AppError` centralizado que implementa `IntoResponse`:

```rust
pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    Database(sqlx::Error),
}
```

## Estado Compartido

El estado de la aplicación se comparte via Axum `State`:

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
}
```
