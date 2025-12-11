use tower_http::cors::{CorsLayer, AllowOrigin};
use std::str::FromStr;

/// Configurar CORS para desarrollo y producción
pub fn create_cors_layer(environment: &str) -> CorsLayer {
    match environment {
        "development" => {
            // Desarrollo: permitir localhost en diferentes puertos y orígenes Tauri
            CorsLayer::permissive()
                .allow_origin(
                    AllowOrigin::list(vec![
                        // Frontend Web local
                        "http://localhost:5173"
                            .parse()
                            .expect("Invalid origin"),
                        "http://localhost:5174"
                            .parse()
                            .expect("Invalid origin"),
                        // Tauri dev
                        "http://localhost:1430"
                            .parse()
                            .expect("Invalid origin"),
                        "tauri://localhost"
                            .parse()
                            .expect("Invalid origin"),
                        // Swagger UI
                        "http://localhost:3000"
                            .parse()
                            .expect("Invalid origin"),
                        // IP local (común en testing)
                        "http://127.0.0.1:5173"
                            .parse()
                            .expect("Invalid origin"),
                        "http://127.0.0.1:3000"
                            .parse()
                            .expect("Invalid origin"),
                    ]),
                )
                .allow_methods(tower_http::cors::AllowMethods::any())
                .allow_headers(tower_http::cors::AllowHeaders::any())
                .allow_credentials(true)
        }
        "production" => {
            // Producción: solo orígenes explícitos
            CorsLayer::permissive()
                .allow_origin(
                    AllowOrigin::list(vec![
                        // Frontend producción (ajustar según tu dominio)
                        "https://courier-app.example.com"
                            .parse()
                            .expect("Invalid origin"),
                        // Tauri desktop app
                        "tauri://tauri.localhost"
                            .parse()
                            .expect("Invalid origin"),
                    ]),
                )
                .allow_methods(tower_http::cors::AllowMethods::any())
                .allow_headers(tower_http::cors::AllowHeaders::any())
                .allow_credentials(true)
        }
        _ => {
            // Fallback: mismo comportamiento que desarrollo
            CorsLayer::permissive()
                .allow_origin(AllowOrigin::list(
                    vec!["http://localhost:5173".parse().expect("Invalid origin")],
                ))
                .allow_methods(tower_http::cors::AllowMethods::any())
                .allow_headers(tower_http::cors::AllowHeaders::any())
                .allow_credentials(true)
        }
    }
}
