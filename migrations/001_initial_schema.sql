-- Crear tipos ENUM si no existen
CREATE TYPE rol_usuario AS ENUM ('cliente', 'transportista', 'admin');
CREATE TYPE estado_pedido AS ENUM ('pendiente', 'confirmado', 'en_transito', 'entregado', 'cancelado');
CREATE TYPE estado_factura AS ENUM ('pendiente', 'pagada', 'vencida', 'cancelada');
CREATE TYPE estado_pago AS ENUM ('pendiente', 'completado', 'fallido', 'reembolsado');
CREATE TYPE metodo_pago_enum AS ENUM ('tarjeta_credito', 'tarjeta_debito', 'transferencia', 'efectivo', 'billetera_digital');
CREATE TYPE estado_transportista AS ENUM ('verificacion_pendiente', 'activo', 'inactivo', 'suspendido');

-- Esta es una migración de referencia basada en el schema existente en Supabase
-- El schema actual está en la base de datos, pero esta migración
-- documenta la estructura para futuras migraciones incrementales
