use axum::{
  http::StatusCode,
  response::Response,
  middleware::Next,
  extract::Request,
};

pub async fn middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
  println!("test middleware!");
  Ok(next.run(req).await)
}