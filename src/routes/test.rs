use axum::{Router, routing::{get, post}, body::{Bytes, Body}, http::StatusCode, response::Response, extract::{Path, Request}, middleware, Extension};
use serde::{Deserialize, Serialize};
use crate::middlewares::{self, header_auth_check::LoginedUser};

use super::CommonResponse;

#[derive(Debug, Serialize, Deserialize)]
struct PostRequestRoutePayload {
  name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
  pub name: String,
  pub age: u32,
  pub habits: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetRequestRoutePayload {
  page: u32,
  size: u32,
  search: Option<String>,
  b: Option<Vec<String>>,
}

pub fn routes() -> Router {
  Router::new()
    .route("/", get(root_route))
    .route("/post-request", post(post_request_route))
    .route("/get-request", get(get_request_route))
    .route("/path-param-and-header-check/:id/:mode", get(path_param_and_header_check_route))
    .route_layer(middleware::from_fn(middlewares::test_only::middleware))
}
async fn root_route(Extension(user): Extension<LoginedUser>) -> &'static str {
  println!("middleware 에서 전달 받은 user : {:?}", user);
  "test root"
}

async fn post_request_route(body: Bytes) -> Response {
  let body_string_result = String::from_utf8(body.to_vec());
  if let Err(_) = body_string_result {
    return Response::builder()
      .status(StatusCode::BAD_REQUEST)
      .header("Content-Type", "application/json")
      .body(Body::from(
        CommonResponse::<String>::new()
          .set_error_code("100100")
          .set_message("요청 본문을 읽는 도중 에러가 발생하였습니다.")
          .to_json_string()
        ))
      .unwrap();
  }
  let body_string = body_string_result.unwrap();
  let req_payload_result = serde_json::from_str::<PostRequestRoutePayload>(body_string.as_str());
  if let Err(_) = req_payload_result {
    return Response::builder()
      .status(StatusCode::BAD_REQUEST)
      .header("Content-Type", "application/json")
      .body(Body::from(
        CommonResponse::<String>::new()
          .set_error_code("100110")
          .set_message("요청 본문을 객체로 변환 도중 에러가 발생하였습니다.")
          .to_json_string()
        ))
      .unwrap();
  }
  let req_payload = req_payload_result.unwrap();
  println!("req_payload : {:?}", req_payload);

  Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .header("Content-Type", "application/json")
    .body(Body::from(
      CommonResponse::new()
        .set_message("정상 응답")
        .set_data(Data {
          name: "홍길동".to_string(),
          age: 31,
          habits: vec!["개발".to_string(), "드라마".to_string()],
        })
        .to_json_string()
      ))
    .unwrap()
}

async fn get_request_route(req: Request) -> Response {
  let mut query_option: Option<GetRequestRoutePayload> = None;
  // let query_str_o = req.uri().query();
  if let Some(query_str) = req.uri().query() {
    if let Ok(payload) = serde_qs::from_str::<GetRequestRoutePayload>(query_str) {
      query_option = Some(payload);
    } else {
      println!("query_str : {:?}", query_str);
    }
  }
  
  if let None = query_option {
    return Response::builder()
      .status(StatusCode::BAD_REQUEST)
      .header("Content-Type", "application/json")
      .body(Body::from(
        CommonResponse::<String>::new()
          .set_error_code("200200")
          .set_message("url query string 규격이 올바르지 않습니다.")
          .to_json_string()
        ))
      .unwrap();
  }
  
  let query = query_option.unwrap();
  println!("query : {:?}", query);

  Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .header("Content-Type", "application/json")
    .body(Body::from(
      CommonResponse::new()
        .set_message("정상 응답")
        .set_data(Data {
          name: "홍길동".to_string(),
          age: 31,
          habits: vec!["개발".to_string(), "드라마".to_string()],
        })
        .to_json_string()
      ))
    .unwrap()
}

async fn path_param_and_header_check_route(
  Path((id, mode)): Path<(u64, String)>, 
  req: Request
) -> Response {
  println!("path_param_and_header_check_route!!");
  println!("id : {:?}", id);
  println!("mode : {:?}", mode);
  let headers = req.headers();
  println!("headers : {:?}", headers);

  Response::builder()
    .status(StatusCode::BAD_REQUEST)
    .header("Content-Type", "application/json")
    .body(Body::from(
      CommonResponse::new()
        .set_message("정상 응답")
        .set_data(Data {
          name: "홍길동".to_string(),
          age: 31,
          habits: vec!["개발".to_string(), "드라마".to_string()],
        })
        .to_json_string()
      ))
    .unwrap()
}