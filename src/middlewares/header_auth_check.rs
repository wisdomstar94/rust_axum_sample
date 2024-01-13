use axum::{
  http::StatusCode,
  response::Response,
  middleware::Next,
  extract::Request,
};

#[derive(Clone, Debug)]
pub struct LoginedUser {
  pub name: String,
  pub created_at: i64,
}

pub async fn middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
  let auth_header = req.headers()
    .get("Authorization")
    .and_then(|header| header.to_str().ok());

  if let Some(authorization) = auth_header {
    println!("authorization : {:?}", authorization);
    let user = LoginedUser {
      name: "홍길동".to_string(),
      created_at: 1705153578473,
    };
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
  } else {
    return Err(StatusCode::UNAUTHORIZED);
  }
}