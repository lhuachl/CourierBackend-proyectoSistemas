# Frontend para el Backend — React (Vite) + Tauri

## Resumen ejecutivo

Objetivo: Single codebase multiplataforma (Windows/Mac/Linux + web fallback) con React (Vite) + Tauri.

Enfoque: Arquitectura basada en features (cada feature contiene UI, hooks, types, services y tests). El API client se genera desde el OpenAPI del backend.

Prioridad inicial: MVP funcional (autenticación, catálogo, checkout, pedidos, perfil, direcciones); posteriormente, panel admin y features nativos.

---

## 1. Estructura del proyecto (feature-based)

Raíz propuesta:

```
src/
  features/
    auth/
      components/
      pages/
      hooks/
      services/
      types.ts
      tests/
    productos/
    pedidos/
    carrito/
    direcciones/
    usuarios/   # admin
    perfil/
    dashboard/
    settings/
  shared/
    ui/        # design system: atoms, molecules
    api/       # client generado, axios/fetch wrapper
    hooks/     # useAuth, useToast, etc.
    libs/      # validation schemas
    stores/    # auth minimal context
    tauri/     # wrappers para APIs nativas
  routes/
  pages/_app.tsx
  main.tsx
  types/
  assets/
  openapi/    # OpenAPI spec + scripts de generación
vite.config.ts
tauri.conf.json
package.json
.env.*
```

---

## 2. Archivo `.env` (ejemplo)

> Nota: variables sensibles deben guardarse en el gestor de secretos del CI o en el sistema operativo. Usar `.env.development` para desarrollo local.

```env
# Frontend & Tauri
VITE_APP_API_BASE=https://api.example.com
VITE_APP_SUPABASE_URL=https://xxxx.supabase.co
VITE_APP_SUPABASE_ANON_KEY=pk.xxx
VITE_APP_OPENAPI_URL=https://api.example.com/openapi.json

# Feature toggles
VITE_FEATURE_ENABLE_REALTIME=true
VITE_FEATURE_ENABLE_NATIVE_PRINT=true

# Maps / External services
VITE_MAPBOX_KEY=pk.xxx
VITE_GOOGLE_MAPS_KEY=xxxx

# Sentry / Analytics
VITE_SENTRY_DSN=https://...
VITE_GA_MEASUREMENT_ID=G-XXXX

# Tauri (valores para el runtime Rust, no exponer al cliente web)
TAURI_RUST_ENV_SUPABASE_JWT_SECRET=xxxxx
TAURI_ALLOWLIST_HTTP=true

# Local DB (Tauri)
VITE_SQLITE_DB_PATH=app_db.sqlite

# Metadata
VITE_APP_NAME=CourierApp
VITE_APP_VERSION=0.1.0
```

Seguridad: nunca publicar claves privadas ni el `SUPABASE_SERVICE_ROLE` en el frontend. Guardar secretos en CI/Secrets.

---

## 3. Dependencias recomendadas (frontend)

- Core / UI:
  - react, react-dom, react-router-dom, vite
  - tailwindcss (o la UI library elegida)
  - headlessui / radix-ui / mantine / chakra (elige una)
- Data fetching / state:
  - @tanstack/react-query
  - react-hook-form + zod
  - axios (o ky)
- OpenAPI & types:
  - openapi-typescript (o swagger-typescript-api / openapi-generator)
- Utilidades:
  - date-fns, clsx, i18next, uuid
- Testing / Dev:
  - vitest, @testing-library/react, msw, prettier, eslint
- Tauri (frontend):
  - @tauri-apps/api

Ejemplo mínimo de `package.json` (dependencias clave):

```json
{
  "dependencies": {
    "react": "^18.x",
    "react-dom": "^18.x",
    "react-router-dom": "^6.x",
    "@tanstack/react-query": "^4.x",
    "axios": "^1.x",
    "react-hook-form": "^7.x",
    "zod": "^3.x",
    "tailwindcss": "^3.x",
    "@tauri-apps/api": "^2.x"
  },
  "devDependencies": {
    "vite": "^5.x",
    "typescript": "^5.x",
    "openapi-typescript": "^14.x",
    "vitest": "^0.x",
    "@testing-library/react": "^14.x",
    "msw": "^1.x"
  }
}
```

---

## 4. Rust / Tauri (crates y plugins recomendados)

- Requisitos: Rust toolchain, tauri-cli, Node.js.
- Plugins/crates sugeridos para la app Tauri:
  - tauri (core) con feature `api-all`
  - tauri-plugin-updater (auto-update)
  - tauri-plugin-store (persistencia simple)
  - tauri-plugin-notification (notificaciones nativas)
  - tauri-plugin-secure-storage o keyring (guardar tokens seguros)
  - opcional: tauri-plugin-sql para SQLite local
  - serde, serde_json, reqwest (si el backend se consulta desde Rust)

---

## 5. Generación de API client y types

- Generar TypeScript types y client desde OpenAPI del backend ayuda a acelerar el desarrollo y mantener tipos.
- Comando sugerido en `package.json`:

```json
"scripts": {
  "gen:api": "openapi-typescript $VITE_APP_OPENAPI_URL --output src/types/api.ts"
}
```

- Para generar client HTTP completo usar `swagger-typescript-api` o `openapi-generator` según preferencias.

---

## 6. Implementación detallada por módulo (qué debe incluir, librerías y patrones)

### A) Auth (feature: `auth`)
- Pages: Login, Register (si aplica), Logout, Profile
- Components: OAuth buttons, LoginForm
- Hooks: `useAuth` (user, token, login/logout)
- Services: `authService` (wrap Supabase client + `/auth/me`)
- Storage: tokens en SecureStorage (Tauri) o `localStorage` en web
- Guards: `ProtectedRoute`, `RoleGuard`
- Librerías: `@supabase/supabase-js`, `react-hook-form`, `zod`, `@tauri-apps/api`

> Nota: El backend valida JWT emitidos por Supabase; usar `/auth/me` para sincronizar perfil y roles.

### B) Productos (feature: `productos`)
- Pages: Catálogo (grid + filters), ProductoDetalle
- Components: `ProductCard`, `ProductGallery`, `Filters`
- Hooks: `useProducts` (paginado) con React Query
- Forms (admin): `ProductEditor` con subida de imágenes
- Librerías: `@tanstack/react-query`, `react-dropzone` o integración con Supabase Storage

### C) Carrito + Checkout (feature: `carrito`, `pedidos`)
- `Cart` context (persistido localmente / Tauri store)
- Pages: Checkout, OrderConfirmation
- Hooks: `useCheckout` (optimistic updates)
- Integración de pago: Stripe sandbox o mock
- Native: impresión de recibo con API nativa de Tauri

### D) Pedidos (feature: `pedidos`)
- Pages: Mis pedidos (paginado), OrderDetail
- Admin pages: Todos los pedidos, cambiar estado, asignar transportista
- Hooks: `useOrders`, `useOrderMutation`
- Real-time: Supabase Realtime o WebSocket para updates de estado

### E) Direcciones (feature: `direcciones`)
- CRUD de direcciones, integración con autofill (Google Places / Mapbox)
- Validación con `zod` y `react-hook-form`

### F) Usuarios / Admin (feature: `usuarios`)
- Listado paginado, edición de rol/estado
- Guard admin
- Export CSV nativo (Tauri) y generación de PDF de facturas

### G) Perfil cliente / transportista
- Visualización y edición, historial de pedidos, subida de foto de perfil

### H) Notificaciones / Nativo (feature: `notifications`)
- Notificaciones de escritorio con plugin Tauri y fallback Push en web
- Toaster in-app y badges, integración con tray icon y auto-launch opcional

### I) Offline & Sync
- Local DB (SQLite en Tauri) para caché y operaciones offline (carrito, borradores)
- Sincronización background al reconectar; política de resolución de conflictos simple (last-write-wins)

### J) Imágenes y archivos
- Upload directo a Supabase Storage o mediante presigned URLs
- Preview y edición básica (crop)
- Export nativo de CSV/PDF via API de Tauri

---

## 7. UI / UX / Design System

- Tailwind CSS + Radix UI / Mantine / Chakra
- Storybook para componentes
- Soporte de tema (light/dark), accesibilidad (a11y) e i18n (i18next)

---

## 8. Testing & QA

- Unit: Vitest + Testing Library
- Mocks para desarrollo y tests: MSW (Mock Service Worker)
- E2E: Playwright (incluir flujos Tauri en CI si es necesario)
- CI: lint, unit tests, generación de client OpenAPI

---

## 9. CI/CD y empaquetado

- Pipeline ejemplo (GitHub Actions):
  - pasos: install, `gen:api`, lint, test, build web, build tauri (opcional), publicar artefactos
- Releases: usar `tauri build` para crear instaladores y usar `tauri-plugin-updater` para auto-update.
- Code signing: firmar builds para Windows y macOS en producción.

---

## 10. Seguridad

- Guardar tokens en `tauri-plugin-secure-storage` o keyring; no exponer secrets en el frontend
- Configurar CSP para la webview
- Limitar permisos en `tauri.conf.json` (allowlist)
- Validación server-side y escape de datos en el cliente

---

## 11. Developer Experience (rápido y sin complicaciones)

- Scripts recomendados:
  - `dev:web`: `vite`
  - `dev:desktop`: `tauri dev`
  - `gen:api`: regenerar tipos desde OpenAPI
  - `storybook`
- Scaffolding:
  - plantilla para features (component + page + hook + service + test)
  - generador (plop) para crear features rápido
- Mock server (MSW) para desarrollo sin backend

---

## 12. Timeline y milestones (ejemplo)

- Sprint 0 (1 semana): Setup repo, Vite + Tauri, design system, gen:api, auth básica
- Sprint 1 (2 semanas): Login, catálogo, detalle producto, carrito, checkout mínimo
- Sprint 2 (2 semanas): Pedidos (mis pedidos), direcciones, profile, payments mock
- Sprint 3 (2 semanas): Admin (users/products), product CRUD, uploads
- Sprint 4 (2 semanas): Offline sync, native features (printing, notifications), polish + tests
- Entrega MVP: ~7 semanas (ajustable)

---

## 13. Features “wow” para impresionar al cliente

- Offline-first con SQLite local y sync transparente
- Live tracking de pedidos en mapa (Mapbox) con ETA y rutas
- Generación nativa de PDF e impresión directa desde la app
- Auto-update y native installers con tray icon y background sync
- Escaneo/Generación de QR para confirmación de entrega
- Integración con hardware (impresora térmica, lector de códigos)
- Demo mode (MSW) para presentar sin backend

---

## 14. Checklist inicial

- [ ] Generar OpenAPI client y tipos
- [ ] Crear scaffold feature template
- [ ] Implementar auth + `useAuth` + Guards
- [ ] Implementar productos + catálogo (paginado)
- [ ] Implementar carrito y checkout mínimo
- [ ] Configurar Tauri dev workflow y secure storage
- [ ] Configurar CI (gen:api, lint, test, build)
- [ ] Preparar scripts de release y signing

---

## 15. Próximos pasos (opciones que puedo generar ahora)

- Scaffold inicial de `src/features/*` con archivos base
- `package.json` y `tauri.conf.json` de ejemplo
- Un `.env.example` listo para el repositorio

Indica cuál de las opciones quieres que genere y la creo inmediatamente.