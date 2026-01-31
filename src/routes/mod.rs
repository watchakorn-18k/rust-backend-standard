use axum::Router;
use crate::state::AppState;

pub mod user_routes;
pub mod auth_routes;
pub mod ws_routes;

pub fn init_routes(state: AppState) -> Router<AppState> {
    let v1_routes = Router::new()
        .merge(auth_routes::auth_routes(state.clone()))
        .merge(user_routes::user_routes(state.clone()));

    let mut app = Router::new()
        .merge(ws_routes::ws_routes(state.clone()))
        .nest("/api/v1", v1_routes)
        .route("/", axum::routing::get(|| async { 
            axum::Json(serde_json::json!({ 
                "message": "Welcome to fdlp Rust Backend Standard API", 
                "version": "0.1.0", 
                "docs": "/docs" 
            })) 
        }))
        .route("/health", axum::routing::get(crate::handlers::health::health_check));

    if state.config.app_mode == "development" {
        app = app
            .route("/docs", axum::routing::get(crate::handlers::docs::scalar_ui))
            .route("/swagger.yaml", axum::routing::get(crate::handlers::docs::swagger_yaml))
            .route("/schema", axum::routing::get(crate::handlers::docs::schema_html))
            .route("/database-schema.mermaid", axum::routing::get(crate::handlers::docs::database_schema_mermaid));
    }

    app
}
