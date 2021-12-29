use crate::components::row_html::RawHTML;
use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{html, Component, Context, Html, Properties};

const MARKDOWN_URL: &str = "https://raw.githubusercontent.com/mayone-du/mayone-du/master/README.md";
const INCORRECT_URL: &str =
  "https://raw.githubusercontent.com/mayone-du/mayone-du/master/README.md";

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ArticleList {
  markdown: FetchState<String>,
}

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
  GetMarkdown,
  GetError,
}

async fn fetch_markdown(url: &'static str) -> Result<String, FetchError> {
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

impl Component for ArticleList {
  type Message = Msg;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      markdown: FetchState::NotFetching,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SetMarkdownFetchState(fetch_state) => {
        self.markdown = fetch_state;
        true
      }
      Msg::GetMarkdown => {
        ctx.link().send_future(async {
          match fetch_markdown(MARKDOWN_URL).await {
            Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
            Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
          }
        });
        ctx
          .link()
          .send_message(Msg::SetMarkdownFetchState(FetchState::Fetching));
        false
      }
      Msg::GetError => {
        ctx.link().send_future(async {
          match fetch_markdown(INCORRECT_URL).await {
            Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
            Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
          }
        });
        ctx
          .link()
          .send_message(Msg::SetMarkdownFetchState(FetchState::Fetching));
        false
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    match &self.markdown {
      FetchState::NotFetching => html! {
          <>
              <button onclick={ctx.link().callback(|_| Msg::GetMarkdown)}>
                  { "Get Markdown" }
              </button>
              <button onclick={ctx.link().callback(|_| Msg::GetError)}>
                  { "Get using incorrect URL" }
              </button>
          </>
      },
      FetchState::Fetching => html! { "Fetching" },
      FetchState::Success(data) => html! { <RawHTML inner_html={data.clone()} /> },
      FetchState::Failed(err) => html! { err },
    }
  }
}
