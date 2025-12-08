use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn get_user(State(_db): State<sqlx::PgPool>, id: Uuid) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({ "user_id": id })))
}
