use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::utils::time::now_bangkok;

pub async fn logger_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path();

    // Skip logging for these paths (e.g., file uploads, health checks)
    let skip_paths = ["/api/v1/users/update-image-profile", "/health"];
    let should_skip = skip_paths.iter().any(|&p| path == p);
    
    let response = next.run(request).await;
    
    if !should_skip {
        let latency = start.elapsed();
        let status = response.status();
        let time = now_bangkok().format("%Y-%m-%dT%H:%M:%S+07:00");

        println!(
            "{} \x1b[1;32m{}\x1b[0m {} \x1b[1;33m{}\x1b[0m \x1b[1;34m{:?}\x1b[0m",
            time,
            method,
            uri,
            status.as_u16(),
            latency
        );
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use tower::ServiceExt;
    use axum::http::Request;

    #[tokio::test]
    async fn test_logger_middleware() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(logger_middleware));

        let response = app
            .oneshot(Request::builder().uri("/").body(axum::body::Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
