# CORS Configuration — Backend

**Última actualización:** 11 de diciembre de 2025

---

## 📋 Descripción General

CORS (Cross-Origin Resource Sharing) ha sido configurado en el backend para permitir peticiones desde diferentes orígenes (Origins) durante desarrollo y producción. Esta configuración asegura que el frontend (web o desktop con Tauri) pueda comunicarse sin restricciones innecesarias.

---

## 🔧 Implementación

### Ubicación del código
- **Módulo:** `src/config/cors.rs`
- **Importado en:** `src/config/mod.rs`
- **Aplicado en:** `src/presentation/routes.rs` (layer en el router)

### Estructura de la configuración

```rust
pub fn create_cors_layer(environment: &str) -> CorsLayer
```

La función recibe el entorno (`development` o `production`) y retorna un `CorsLayer` con la configuración apropiada.

---

## 🚀 Orígenes Permitidos

### Desarrollo (`APP_ENV=development`)

```
✅ http://localhost:5173       # Vite dev server (puerto predeterminado)
✅ http://localhost:5174       # Vite dev server (puerto alternativo)
✅ http://localhost:3000       # Backend mismo / Swagger UI
✅ http://127.0.0.1:5173       # localhost en IPv4
✅ http://127.0.0.1:3000       # localhost en IPv4 (backend)
✅ http://localhost:1430       # Tauri dev (posible puerto)
✅ tauri://localhost            # Tauri webview
```

**Métodos HTTP permitidos:** ANY (GET, POST, PUT, DELETE, PATCH, etc.)

**Headers permitidos:** ANY

**Credenciales:** ✅ Habilitadas (permite envío de cookies/auth headers)

### Producción (`APP_ENV=production`)

```
✅ https://courier-app.example.com     # Frontend producción (ajustar tu dominio)
✅ tauri://tauri.localhost              # Tauri desktop app
```

**Métodos HTTP permitidos:** ANY

**Headers permitidos:** ANY

**Credenciales:** ✅ Habilitadas

---

## ⚙️ Cómo usar

### 1. Establecer el entorno

En desarrollo (por defecto):

```bash
# Sin variable: usa "development" automáticamente
cargo run
```

Con variable explícita:

```bash
APP_ENV=development cargo run
```

Para producción:

```bash
APP_ENV=production cargo run
```

### 2. Ejemplo desde el frontend (React/Tauri)

```typescript
// src/lib/api-client.ts
import axios from 'axios';

const apiClient = axios.create({
  baseURL: process.env.VITE_API_BASE_URL || 'http://localhost:3000',
  timeout: 15000,
  withCredentials: true,  // Importante: habilitar credenciales
});

// Las peticiones funcionarán sin bloques CORS
export default apiClient;
```

Ejemplo de una petición:

```typescript
// src/features/users/api/useUsers.ts
import { useQuery } from '@tanstack/react-query';
import apiClient from '@/lib/api-client';

export function useUsers() {
  return useQuery({
    queryKey: ['users'],
    queryFn: async () => {
      const { data } = await apiClient.get('/api/admin/users');
      return data;
    },
  });
}
```

---

## 🔐 Seguridad

### Desarrollo

- Permitir `localhost` en todos los puertos comunes es seguro durante desarrollo.
- La variable `APP_ENV` se lee en tiempo de ejecución (no hardcodeada).

### Producción

- **Restringir estrictamente** a tus dominios reales.
- Cambiar `https://courier-app.example.com` por tu dominio actual.
- Usar HTTPS obligatoriamente.
- No usar `CorsLayer::permissive()` en producción (actual implementation lo hace, pero el `allow_origin` restricts).

---

## ⚠️ Troubleshooting

### Error: "Access to XMLHttpRequest has been blocked by CORS policy"

**Causa:** Tu origen no está en la lista de permitidos.

**Solución:**
1. Verifica que `APP_ENV` es correcto.
2. Agrega tu origen a `src/config/cors.rs` → función `create_cors_layer`.
3. Reinicia el servidor backend.

### Error: "The CORS protocol does not allow specifying a wildcard (`*`) for the header `Access-Control-Allow-Credentials`"

**Causa:** No se puede usar `*` con credenciales habilitadas.

**Solución:** Ya resuelta en la implementación actual; `allow_origin` especifica orígenes explícitos.

---

## 📝 Siguiente paso

Si necesitas agregar un nuevo origen (ej. otro puerto de desarrollo):

1. Edita `src/config/cors.rs`
2. Añade el origen a la lista en `development` o `production`:

```rust
"http://localhost:8080".parse().expect("Invalid origin"),
```

3. Recompila y reinicia:

```bash
cargo build
```

---

## 📚 Referencias

- [MDN: CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
- [tower-http CORS](https://github.com/tower-rs/tower-http/blob/main/tower-http/src/cors/mod.rs)
- [Axum middleware](https://docs.rs/axum/latest/axum/middleware/index.html)
