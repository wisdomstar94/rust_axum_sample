use axum::{Router, middleware};
use routes::{root, test};
pub mod routes;
pub mod middlewares;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .nest("/", root::routes())
    .nest("/test", test::routes())
    .route_layer(middleware::from_fn(middlewares::header_auth_check::middleware));
  
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3336").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}