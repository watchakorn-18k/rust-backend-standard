use crate::{handlers::user_handler::UserHandler, state::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn user_routes(state: AppState) -> Router<AppState> {
    let auth = axum::middleware::from_fn_with_state(state.clone(), crate::middlewares::auth::auth_middleware);

    Router::new()
        .nest("/users", Router::new()
            .route("/", post(UserHandler::create_user).get(UserHandler::list_users).route_layer(auth.clone()))
            .route("/:id", get(UserHandler::get_user).put(UserHandler::update_user).route_layer(auth))
        )
}
