# Frontend for Backend — CourierApp Client (Omnicanal)

**Versión:** 2.0.0 · **Stack:** Tauri v2 + React + TypeScript (strict) · **Targets:** Web (SPA), Desktop (Win/Mac/Linux), Mobile (iOS/Android)

---

## 1. Visión Ejecutiva

Objetivo

Implementar una interfaz unificada (single codebase) que consuma los 49 endpoints actuales del Backend, priorizando velocidad de desarrollo y robustez operativa.

Filosofía de diseño

- Minimalismo radical: no añadir dependencias que no resuelvan un problema crítico.
- Server-State First: la UI replica la caché del servidor; minimizar estado global en el cliente.
- TDD-Kanban: desarrollo guiado por pruebas de aceptación (no por historias vagas).
- Offline-ready: soporte para operaciones en zonas sin cobertura (SQLite local + optimistic updates).

---

## 2. "Killer" Tech Stack (justificación)

| Capa | Tecnología | Justificación técnica |
|------|------------|-----------------------|
| Core & Runtime | Tauri v2 + Vite | Unifica web y binarios nativos; tamaño de binarios reducido; permite build mobile con adaptaciones. |
| Lenguaje | TypeScript (strict) | Reduce bugs en tiempo de compilación; tipos generados desde OpenAPI. |
| UI Framework | React 18+ | Ecosistema maduro y rendimiento. |
| Estilos & UI | TailwindCSS + shadcn/ui | Sin runtime CSS-in-JS; componentes accesibles y reutilizables. |
| Data fetching | TanStack Query v5 | Caché, reintentos, sincronización y estrategias offline-first. |
| State manager | Zustand | Minimalista para el poco estado global necesario. |
| Routing | TanStack Router | Enrutamiento type-safe. |
| Forms | React Hook Form + Zod | Rendimiento y validación compartida con backend. |

---

## 3. Configuración del entorno (.env) — ejemplo

> Nota: no subir secretos al repositorio. Usar vault/secrets en CI.

```env
# =================================================================
# 🌐 API & NETWORK
# =================================================================
VITE_API_BASE_URL=https://api.tu-courier-backend.com/v1
VITE_API_TIMEOUT=15000
VITE_API_RETRY_COUNT=2

# =================================================================
# 🔐 SECURITY & AUTH
# =================================================================
VITE_AUTH_STORAGE_KEY=courier_auth_token_v1
VITE_PUBLIC_ENCRYPTION_KEY=-----BEGIN PUBLIC KEY-----...  

# =================================================================
# 🚀 FEATURE FLAGS (control de módulos)
# =================================================================
VITE_FEATURE_AUTH=true
VITE_FEATURE_USERS=true
VITE_FEATURE_ORDERS=true
VITE_FEATURE_PRODUCTS=true
VITE_FEATURE_WAREHOUSES=true
VITE_FEATURE_ADDRESSES=true
VITE_FEATURE_CARRIERS=false
VITE_FEATURE_ZONES=false

# =================================================================
# 📱 PLATFORM BEHAVIOR
# =================================================================
VITE_UI_MODE=auto        # web | desktop | mobile | auto
VITE_LOG_LEVEL=debug
VITE_APP_VERSION=0.9.0-beta
```

---

## 4. Estructura del proyecto (feature-based)

Organización pensada para escalar: cada `feature` contiene UI, hooks, services, tipos y tests.

```
src/
├── app/                  # Providers (QueryClient, AuthProvider, Router)
├── assets/
├── components/           # Atoms y componentes compartidos
│   └── ui/               # shadcn/ui wrappers
├── features/             # Módulos de negocio (mapa directo al backend)
│   ├── auth/
│   ├── users/
│   ├── profiles/
│   ├── orders/
│   ├── products/
│   ├── warehouses/
│   └── addresses/
├── hooks/                # Hooks transversales
├── lib/                  # axios instance, helpers, cn
├── routes/               # Definición de rutas (file-based o central)
└── main.tsx
```

Cada feature incluye:
- `components/` (UI específico)
- `api/` (hooks de React Query: useX, useXMutation)
- `types/` (Zod schemas + tipos TS generados)
- `pages/` (si aplica)
- `tests/` (Vitest + Testing Library)

---

## 5. Metodología: TDD-Kanban

Principio: una feature no existe hasta que su suite de tests pasa.

Flujo Kanban (columnas):
1. Specs (Backlog): escribir tests de aceptación (fallarán inicialmente).
2. Red (Implementation): implementar componentes y hooks; tests pueden fallar.
3. Green (Refine): pasar tests; integración con backend correcta (200 OK).
4. Refactor (UX/UI): pulido visual y limpieza del código.
5. Ship: activar feature flag y mergear a `main`.

Ejemplo (Orders):
- Escribir `OrderList.test.tsx`.
- Implementar `OrderList.tsx` y `useOrders()`.
- Mockear con MSW en tests; pasar tests; aplicar estilos.

---

## 6. Roadmap (alineado al Backend)

### Fase 1 — Core Foundation (prioridad crítica)
- Auth (1 endpoint): login minimalista, persistencia JWT segura, interceptor 401.
- Users (7 endpoints): CRUD en modal, tabla con filtros.
- Profiles (8 endpoints): "Mi cuenta", cambio de password.

### Fase 2 — Logística Operativa
- Products (11 endpoints): catálogo, búsqueda debounced y gestión de imágenes.
- Warehouses (6 endpoints): listado y asignación de inventario.
- Addresses (9 endpoints): formularios inteligentes y mapa.

### Fase 3 — Transaccional
- Orders (6 endpoints): Kanban de estados, timeline de pedidos, impresión nativa de guías.

### Fase 4 — Expansión (pendiente backend)
- Carriers (UI shell preparado).
- Zones (interfaces TS preliminares).

---

## 7. Next steps (inmediatos)

1. Scaffold: inicializar Vite + Tauri + TypeScript.
2. UI: instalar Tailwind + shadcn/ui.
3. Generador de features: script que crea la estructura `src/features/*`.
4. API generation: ejecutar `openapi-typescript` contra el Swagger del backend.
5. Configurar MSW para desarrollo sin backend.

---

## 8. Notas rápidas de implementación

- Generar client Typescript desde OpenAPI y exponer hooks en `features/*/api`.
- Centralizar la instancia HTTP (axios/ky) con interceptores para auth y timeouts.
- Usar React Query para todas las llamadas server-state; persistencia local en Tauri store / SQLite para offline.
- Guardas de rutas y role-based access control (AdminGuard) en `routes/`.
- Integración nativa (Tauri): secure storage, notifications, file export (CSV/PDF).

---

Si quieres, puedo ahora:
- Generar el `template` inicial de `src/features/*` (scaffold).
- Crear un `package.json` y `tauri.conf.json` de ejemplo.
- Generar `.env.example` listo para el repositorio.

Indica cuál prefieres y lo creo inmediatamente.