use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub enum FetchState<T> {
  NotFetching,
  Fetching,
  Success(T),
  Failed(FetchError),
}

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
  err: JsValue,
}
impl Display for FetchError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Debug::fmt(&self.err, f)
  }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
  fn from(value: JsValue) -> Self {
    Self { err: value }
  }
}

pub async fn fetch_row_text<E: From<JsValue>>(url: &str) -> Result<String, E> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(url, &opts)?;

  let window = gloo::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp: Response = resp_value.dyn_into().unwrap();

  let text = JsFuture::from(resp.text()?).await?;
  Ok(text.as_string().unwrap())
}
