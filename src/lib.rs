pub mod config;
pub mod db;
pub mod dtos;
pub mod error;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod providers;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;
pub mod mock;

use axum::Router;
use tower_http::{
    catch_panic::CatchPanicLayer,
    cors::CorsLayer,
    trace::TraceLayer,
};
use axum::http::{header::{CONTENT_TYPE, AUTHORIZATION}, Method, HeaderValue};

pub fn create_app(state: state::AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    routes::init_routes(state.clone())
        .layer(axum::middleware::from_fn(middlewares::logger::logger_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(CatchPanicLayer::new())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::get_mock_state;

    #[tokio::test]
    async fn test_create_app() {
        let state = get_mock_state();
        let _app = create_app(state);
    }
}
