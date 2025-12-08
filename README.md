# Integrador Backend - Rust

Backend moderno construido con Rust, siguiendo una arquitectura Clean Architecture con soporte para tiempo real, integración con PostgreSQL (Supabase) y despliegue en la nube.

## 🏗️ Arquitectura

```
src/
├── domain/                 # Lógica de negocio y entities
│   ├── entities/          # Modelos de dominio (User, Producto, Pedido, etc.)
│   └── repositories/      # Interfaces de repositorios (traits)
│
├── application/           # Casos de uso y servicios
│   ├── dto/               # Data Transfer Objects
│   └── services/          # Lógica de aplicación
│
├── infrastructure/        # Implementaciones de bajo nivel
│   ├── database/          # Configuración de base de datos
│   └── repositories/      # Implementaciones de repositorios
│
├── presentation/          # API HTTP y WebSockets
│   ├── handlers/          # Controladores de rutas
│   ├── middleware/        # Middlewares (autenticación, etc.)
│   └── routes.rs          # Definición de rutas
│
├── config/                # Configuración global
│   ├── env.rs             # Variables de entorno
│   └── database.rs        # Pool de conexiones
│
└── main.rs                # Punto de entrada

migrations/
└── *.sql                  # Migraciones versionadas
```

## 🚀 Inicio rápido

### Requisitos
- Rust 1.70+ (instalar desde https://rustup.rs/)
- PostgreSQL o Supabase
- sqlx-cli (para migraciones)

### Instalación

1. **Clonar y configurar**
```bash
cd integrador
```

2. **Crear archivo `.env`** basado en `.env.example`:
```bash
cp .env.example .env
# Editar .env con tus credenciales de Supabase
```

3. **Instalar sqlx-cli** (si aún no lo has hecho):
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

4. **Ejecutar migraciones**:
```bash
sqlx migrate run
```

5. **Compilar y ejecutar**:
```bash
cargo run
```

El servidor estará disponible en `http://0.0.0.0:3000`

## 📦 Stack Tecnológico

- **Framework Web**: Axum (rápido, modular, built on Tokio)
- **Runtime Asíncrono**: Tokio (para concurrencia y tiempo real)
- **Base de Datos**: SQLx + PostgreSQL (Supabase)
- **Serialización**: Serde
- **Autenticación**: JWT (futuro)
- **WebSockets**: Axum + Tokio (para tiempo real)
- **Logging**: Tracing + Tracing-Subscriber
- **Entorno**: Dotenvy

## 🔄 Flujo de Desarrollo

### 1. Definir entity (Domain)
Crear nuevas entidades en `src/domain/entities/`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MiEntity {
    pub id: Uuid,
    // campos...
}
```

### 2. Definir repository trait (Domain)
Crear traits en `src/domain/repositories/`

```rust
#[async_trait]
pub trait MiRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MiEntity>, sqlx::Error>;
}
```

### 3. Implementar repository (Infrastructure)
Crear implementación en `src/infrastructure/repositories/`

```rust
#[async_trait]
impl MiRepository for MiRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MiEntity>, sqlx::Error> {
        sqlx::query_as::<_, MiEntity>("SELECT * FROM mi_tabla WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}
```

### 4. Crear DTOs (Application)
Crear DTOs en `src/application/dto/`

### 5. Crear servicios (Application)
Crear lógica en `src/application/services/`

### 6. Crear handlers (Presentation)
Crear endpoints en `src/presentation/handlers/`

### 7. Registrar rutas (Presentation)
Agregar rutas en `src/presentation/routes.rs`

## 🗄️ Migraciones

Las migraciones están versionadas en `migrations/` y se ejecutan con `sqlx migrate run`.

Para crear una nueva migración:
```bash
sqlx migrate add -r <nombre_migracion>
```

## 🔐 Estructura de BD (Supabase)

El proyecto está diseñado para trabajar con el schema actual en Supabase. Las inconsistencias identificadas se refactorizarán con migraciones incrementales conforme avanza el desarrollo.

### Tablas principales
- **users**: Autenticación y datos básicos
- **perfiles_cliente**: Información de clientes
- **productos**: Catálogo de productos
- **pedidos**: Órdenes de envío
- **direcciones**: Direcciones de entrega
- **transportistas**: Datos de transportistas
- **facturas**: Facturación
- **pagos**: Registros de pagos
- **zonas**: Zonas de entrega
- **evento_pedidos**: Auditoría de cambios en pedidos

## ⚡ Características de Tiempo Real

Para WebSockets y tiempo real:
- Usar Axum extractors con `ws::upgrade`
- Implementar handlers que escuchen cambios en BD
- Usar Tokio channels para broadcast de eventos

Ejemplo (futuro):
```rust
// En handlers
pub async fn subscribe_pedido_updates(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}
```

## 🐳 Despliegue

### Docker
```bash
docker build -t integrador-backend .
docker run -p 3000:3000 --env-file .env integrador-backend
```

### Supabase (Cloud)
- Base de datos PostgreSQL estándar
- Integración directa con Railway, Fly.io, AWS, GCP

## 📚 Recursos Útiles

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://tokio.rs/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Rust Book](https://doc.rust-lang.org/book/)

## 🤝 Notas Importantes

1. **Database First**: El schema se define primero en Supabase
2. **Iterativo**: Se refactoriza conforme surge la necesidad
3. **Type Safety**: Aprovecha el compilador de Rust para validar en tiempo de compilación
4. **Async First**: Todo es asíncrono para máxima performance

---

**Última actualización**: 2025-12-08
