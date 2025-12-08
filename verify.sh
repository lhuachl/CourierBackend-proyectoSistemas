#!/bin/bash

echo "🔍 Verificando setup de Integrador Backend..."
echo ""

# Verificar Rust
echo "📦 Versión de Rust:"
rustc --version || echo "❌ Rust no instalado"
echo ""

# Verificar cargo
echo "📦 Versión de Cargo:"
cargo --version || echo "❌ Cargo no instalado"
echo ""

# Verificar estructura
echo "📁 Estructura de carpetas:"
ls -la src/ 2>/dev/null && echo "✅ src/ existe" || echo "❌ src/ no existe"
echo ""

# Verificar compilación
echo "🔨 Compilando proyecto..."
cargo check && echo "✅ Compilación exitosa" || echo "❌ Errores de compilación"
echo ""

# Verificar migraciones
echo "🗄️  Verificando migraciones:"
ls -la migrations/ 2>/dev/null && echo "✅ migrations/ existe" || echo "❌ migrations/ no existe"
echo ""

echo "✅ Verificación completada"
echo ""
echo "📝 Próximos pasos:"
echo "1. Crear archivo .env con la URL de Supabase"
echo "2. Ejecutar: cargo build"
echo "3. Ejecutar: cargo run"
