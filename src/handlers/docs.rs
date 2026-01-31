use axum::{response::Html, response::IntoResponse};

pub async fn scalar_ui() -> impl IntoResponse {
    Html(r#"
<!doctype html>
<html>
  <head>
    <title>Scalar API Reference</title>
    <meta charset="utf-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <script
      id="api-reference"
      data-url="/swagger.yaml"></script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  </body>
</html>
"#)
}

pub async fn swagger_yaml() -> impl IntoResponse {
    let yaml = std::fs::read_to_string("docs/swagger.yaml").unwrap_or_default();
    (
        [("content-type", "text/yaml")],
        yaml,
    )
}
