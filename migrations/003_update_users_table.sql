-- Migración: Actualizar tabla users con campos adicionales
-- Fecha: 11 de diciembre de 2025
-- Descripción: Agrega campos email, activo y otros necesarios para gestión completa de usuarios

-- Primero, verificar si la tabla existe y hacer cambios incrementales
-- Esta migración asume que la tabla 'users' ya existe con los campos básicos

-- 1. Agregar columna email si no existe
ALTER TABLE public.users
ADD COLUMN IF NOT EXISTS email VARCHAR(255) UNIQUE;

-- 2. Agregar columna activo si no existe (por defecto true)
ALTER TABLE public.users
ADD COLUMN IF NOT EXISTS activo BOOLEAN NOT NULL DEFAULT true;

-- 3. Crear índices para optimizar búsquedas
CREATE INDEX IF NOT EXISTS idx_users_email ON public.users(email);
CREATE INDEX IF NOT EXISTS idx_users_rol ON public.users(rol);
CREATE INDEX IF NOT EXISTS idx_users_activo ON public.users(activo);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON public.users(created_at DESC);

-- 4. Agregar constraint para validar rol
ALTER TABLE public.users
ADD CONSTRAINT check_rol_valido 
CHECK (rol IN ('cliente', 'transportista', 'admin'))
NOT VALID;

-- 5. Comentarios en las columnas para documentación
COMMENT ON TABLE public.users IS 'Tabla de usuarios del sistema con datos extendidos sincronizados desde Supabase auth.users';
COMMENT ON COLUMN public.users.id IS 'UUID del usuario, sincronizado desde Supabase auth.users';
COMMENT ON COLUMN public.users.email IS 'Email del usuario, sincronizado desde Supabase auth.users';
COMMENT ON COLUMN public.users.rol IS 'Rol en el sistema: cliente, transportista o admin';
COMMENT ON COLUMN public.users.activo IS 'Estado de la cuenta (true: activo, false: suspendido/eliminado)';
COMMENT ON COLUMN public.users.created_at IS 'Fecha de creación del usuario';
COMMENT ON COLUMN public.users.updated_at IS 'Fecha de última actualización';
