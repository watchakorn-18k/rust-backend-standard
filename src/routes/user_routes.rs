use crate::{handlers::user_handler, state::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(user_handler::create_user).get(user_handler::list_users))
        .route("/:id", get(user_handler::get_user).put(user_handler::update_user))
        .with_state(state) // redundant if nested correctly but safe
}
