mod config;
mod domain;
mod application;
mod infrastructure;
mod presentation;
mod shared;

use axum::Router;
use config::{AppConfig, create_pool};
use presentation::create_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar configuración desde variables de entorno
    let config = AppConfig::from_env();

    // Crear pool de conexiones a la base de datos
    let pool = create_pool(&config.database_url).await?;

    // Crear router con todas las rutas
    let app: Router = create_routes(pool);

    // Crear dirección del servidor
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("🚀 Servidor iniciado en {}", addr);
    println!("📦 Ambiente: {}", config.environment);

    // Iniciar servidor
    axum::serve(listener, app).await?;

    Ok(())
}
