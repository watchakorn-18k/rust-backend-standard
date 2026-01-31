use axum::{
    response::Html,
    routing::get,
    Router,
};
use tower_http::services::ServeFile;

pub fn swagger_routes() -> Router {
    Router::new()
        .route("/", get(scalar_handler))
        .nest_service("/swagger.yaml", ServeFile::new("docs/swagger.yaml"))
}

async fn scalar_handler() -> Html<&'static str> {
    Html(r#"
<!doctype html>
<html>
  <head>
    <title>Rust Backend API</title>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1" />
    <style>
      body {
        margin: 0;
      }
    </style>
  </head>
  <body>
    <script
      id="api-reference"
      data-url="/docs/swagger.yaml"></script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  </body>
</html>
"#)
}
