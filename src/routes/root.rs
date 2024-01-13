use axum::{Router, routing::get};

pub fn routes() -> Router {
  Router::new()
    .route("/", get(root_route))
}

async fn root_route() -> &'static str {
  "hi"
}