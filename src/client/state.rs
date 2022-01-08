use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// データ取得のstate
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

pub enum Msg {
  SetMarkdownFetchState(FetchState<String>),
  FetchStart,
}
