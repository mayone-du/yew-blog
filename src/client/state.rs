use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;

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

pub enum FetchMessage {
  SetFetchState(FetchState<String>),
  FetchStart,
}
