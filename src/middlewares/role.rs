use crate::error::AppError;
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn admin_guard(
    _request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Placeholder logic - any user is allowed for now
    Ok(next.run(_request).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
        middleware,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_admin_guard_always_ok_for_now() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(middleware::from_fn(admin_guard));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
