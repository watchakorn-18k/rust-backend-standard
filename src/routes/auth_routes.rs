use crate::{handlers::auth_handler::AuthHandler, state::AppState};
use axum::{
    routing::post,
    Router,
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", Router::new()
            .route("/register", post(AuthHandler::register))
            .route("/login", post(AuthHandler::login))
        )
        .with_state(state)
}
