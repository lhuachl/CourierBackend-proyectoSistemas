-- Migración para agregar campos faltantes a la tabla users
-- Fecha: 11 de diciembre de 2025

-- Verificar si la columna 'email' existe antes de agregarla
ALTER TABLE public.users 
ADD COLUMN IF NOT EXISTS email VARCHAR(255) UNIQUE,
ADD COLUMN IF NOT EXISTS activo BOOLEAN DEFAULT true NOT NULL;

-- Agregar índices para mejorar performance en búsquedas
CREATE INDEX IF NOT EXISTS idx_users_email ON public.users(email) WHERE activo = true;
CREATE INDEX IF NOT EXISTS idx_users_rol ON public.users(rol) WHERE activo = true;
CREATE INDEX IF NOT EXISTS idx_users_created_at ON public.users(created_at DESC) WHERE activo = true;

-- Comentarios para documentación
COMMENT ON COLUMN public.users.email IS 'Email sincronizado desde Supabase auth.users';
COMMENT ON COLUMN public.users.activo IS 'Estado del usuario (soft delete cuando es false)';
COMMENT ON COLUMN public.users.rol IS 'Rol en el sistema: cliente, transportista, admin';
COMMENT ON COLUMN public.users.created_at IS 'Fecha de creación del usuario';
COMMENT ON COLUMN public.users.updated_at IS 'Fecha de última actualización';
