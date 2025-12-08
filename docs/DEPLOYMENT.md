# 🚀 Despliegue

## Opciones de Despliegue

### 1. Railway (Recomendado para inicio)
```bash
# Conectar repo
railway link

# Configurar variables
railway variables set DATABASE_URL=...
railway variables set SUPABASE_JWT_SECRET=...

# Deploy
railway up
```

### 2. Fly.io
```bash
fly launch
fly secrets set DATABASE_URL=...
fly deploy
```

### 3. Docker + VPS
```bash
docker build -t integrador-backend .
docker run -d -p 3000:3000 --env-file .env integrador-backend
```

### 4. AWS ECS / GCP Cloud Run
Ver guías específicas de cada proveedor.

## Variables de Entorno Requeridas

```env
# Base de datos
DATABASE_URL=postgresql://user:pass@host:5432/db

# Servidor
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
ENVIRONMENT=production

# Supabase Auth
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_JWT_SECRET=tu-secret
```

## Checklist Pre-Producción

- [ ] Variables de entorno configuradas
- [ ] Migraciones ejecutadas
- [ ] CORS configurado para dominio de frontend
- [ ] HTTPS habilitado
- [ ] Rate limiting configurado
- [ ] Logs centralizados
- [ ] Monitoreo (Sentry, etc.)

## CI/CD

Ejemplo con GitHub Actions:

```yaml
name: Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cargo build --release
      - name: Deploy to Railway
        run: railway up
```
