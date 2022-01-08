use crate::client::state::{FetchError, FetchState};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn fetch_row_text(url: &str) -> Result<String, FetchError> {
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
