use std::fmt::Debug;
use chrono::Local;
use serde::{Deserialize, Serialize};

pub mod test;
pub mod root;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonResponse<T: Debug + Serialize> {
  timestamp: i64,
  error_code: Option<String>, // only error
  message: Option<String>, // only error
  data: Option<T>, // only success
}

impl<T> CommonResponse<T> 
where T: Debug + Serialize
{
  pub fn new() -> Self {
    Self {
      timestamp: Local::now().timestamp_millis(),
      error_code: None,
      message: None,
      data: None,
    }
  }

  pub fn set_error_code(mut self, str: &str) -> Self {
    self.error_code = Some(str.to_string());
    self
  }

  pub fn set_message(mut self, str: &str) -> Self {
    self.message = Some(str.to_string());
    self
  }

  pub fn set_data(mut self, data: T) -> Self {
    self.data = Some(data);
    self
  }

  pub fn to_json_string(&self) -> String {
    let result = serde_json::to_string(self);
    if let Ok(string) = result {
      string
    } else {
      let now = Local::now();
      println!("[{:?}, {:?}] json string 화 실패.. timestamp = {:?}", now.timestamp_millis(), now.format("%Y-%m-%d %H:%M:%S"), self.timestamp);
      format!("`{{`\"timestamp\": \"{:?}\", \"error_code\": \"{:?}\", \"message\": \"{:?}\", \"data\": \"{:?}\"`}}`", self.timestamp, self.error_code, self.message, self.data)
    }
  }
}