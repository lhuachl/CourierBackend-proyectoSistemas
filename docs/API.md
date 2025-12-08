# 📡 API Reference

> **Nota**: La documentación interactiva completa está en `/swagger-ui`

## Base URL

- **Desarrollo**: `http://localhost:3000`
- **Producción**: `https://api.tu-dominio.com`

## Autenticación

Todas las rutas protegidas requieren header:
```
Authorization: Bearer <jwt_token>
```

## Endpoints

### Health Check
```
GET /health
Response: "OK"
```

### Auth
Ver [AUTH.md](AUTH.md) para detalles completos.

### Usuarios
```
GET    /api/users/:id      → Obtener usuario
PUT    /api/users/:id      → Actualizar usuario
```

### Productos
```
GET    /api/productos           → Listar productos
GET    /api/productos/:id       → Obtener producto
POST   /api/productos           → Crear producto (admin)
PUT    /api/productos/:id       → Actualizar producto (admin)
DELETE /api/productos/:id       → Eliminar producto (admin)
```

### Pedidos
```
GET    /api/pedidos             → Listar pedidos del usuario
GET    /api/pedidos/:id         → Obtener pedido
POST   /api/pedidos             → Crear pedido
PUT    /api/pedidos/:id/estado  → Actualizar estado
```

### WebSocket (Tiempo Real)
```
WS /ws/pedidos/:id   → Suscribirse a actualizaciones de pedido
WS /ws/transportista → Actualizaciones para transportista
```

## Respuestas

### Éxito
```json
{
  "data": { ... },
  "message": "Operación exitosa"
}
```

### Error
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Recurso no encontrado"
  }
}
```

## Códigos HTTP

| Código | Significado |
|--------|-------------|
| 200 | OK |
| 201 | Creado |
| 400 | Bad Request |
| 401 | No autenticado |
| 403 | Sin permisos |
| 404 | No encontrado |
| 500 | Error interno |
