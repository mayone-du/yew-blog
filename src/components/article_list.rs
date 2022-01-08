use crate::client::fetch_raw_text::{fetch_row_text, FetchState};
use crate::constants::vars::ARTICLE_LIST_META_URL;
use crate::meta::data_list::MetaDataList;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

// component & stat
pub struct ArticleList {
  markdown: FetchState<String>,
}

pub enum Msg {
  SetMarkdownFetchState(FetchState<String>),
  GetMarkdown,
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
          match fetch_row_text(ARTICLE_LIST_META_URL).await {
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
        <button onclick={ctx.link().callback(|_| Msg::GetMarkdown)}>
          { "Get Markdown" }
        </button>
      },
      FetchState::Fetching => html! { "Fetching" },
      FetchState::Success(data) => {
        let json_data: MetaDataList = serde_json::from_str(&data).unwrap();
        web_sys::console::log_1(&data.into());
        json_data
          .map(|meta| {
            html! {
              <div>
                <h1 class="font-bold text-3xl">{meta.title}</h1>
                <p>{meta.description}</p>
                <p>{meta.created_at}</p>
              </div>
            }
          })
          .collect::<Html>()
      }
      FetchState::Failed(err) => html! { err },
    }
  }
}
