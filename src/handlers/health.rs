use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    // Check Mongo
    let mongo_status = match state.db.run_command(mongodb::bson::doc! {"ping": 1}, None).await {
        Ok(_) => "OK",
        Err(_) => "Error",
    };

    // Check Redis
    let redis_status = match state.redis.get("health_check_ping").await {
        Ok(_) => "OK",
        Err(_) => "Error",
    };

    Json(json!({
        "status": "UP",
        "database": mongo_status,
        "redis": redis_status,
    }))
}
