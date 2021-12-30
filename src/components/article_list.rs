use crate::client::fetch_raw_text::{fetch_row_text, FetchError, FetchState};
use crate::components::row_html::RawHTML;
use yew::{html, Component, Context, Html, Properties};

const MARKDOWN_URL: &str = "https://raw.githubusercontent.com/mayone-du/mayone-du/master/README.md";
const INCORRECT_URL: &str =
  "https://raw.githubusercontent.com/mayone-du/mayone-du/master/README.md";

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ArticleList {
  markdown: FetchState<String>,
}

pub enum Msg {
  SetMarkdownFetchState(FetchState<String>),
  GetMarkdown,
  GetError,
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
          match fetch_row_text(MARKDOWN_URL).await {
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
          match fetch_row_text(INCORRECT_URL).await {
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
