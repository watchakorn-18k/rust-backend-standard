use crate::{handlers::auth_handler, state::AppState};
use axum::{
    routing::post,
    Router,
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .with_state(state)
}
