use axum::Router;
use routes::{root, test};
pub mod routes;

#[tokio::main]
async fn main() {
  let app = Router::new()
    .nest("/", root::routes())
    .nest("/test", test::routes());
  
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3336").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}