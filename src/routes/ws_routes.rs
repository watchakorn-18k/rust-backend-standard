use axum::{
    routing::get,
    Router,
};
use crate::{handlers, state::AppState};

pub fn ws_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/ws", get(handlers::ws::ws_handler))
        .with_state(state)
}
