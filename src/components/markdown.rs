use crate::client::{fetch::fetch_row_text, state::FetchMessage, state::FetchState};
use crate::components::row_html::RawHTML;
use pulldown_cmark::{html::push_html, Parser};
use regex;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub request_url: String,
}

pub struct Markdown {
  response: FetchState<String>,
}

impl Component for Markdown {
  type Message = FetchMessage;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_message(FetchMessage::FetchStart);
    Self {
      response: FetchState::NotFetching,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      FetchMessage::SetFetchState(fetch_state) => {
        self.response = fetch_state;
        true
      }
      FetchMessage::FetchStart => {
        let request_url = ctx.props().request_url.clone();
        ctx.link().send_future(async move {
          match fetch_row_text(&request_url).await {
            Ok(md) => FetchMessage::SetFetchState(FetchState::Success(md)),
            Err(err) => FetchMessage::SetFetchState(FetchState::Failed(err)),
          }
        });
        ctx
          .link()
          .send_message(FetchMessage::SetFetchState(FetchState::Fetching));
        false
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let loading = html! {
      <div class="animate-pulse bg-gray-300 w-full h-8"></div>
    };
    match &self.response {
      FetchState::NotFetching => loading,
      FetchState::Fetching => loading,
      FetchState::Success(data) => {
        // メタデータを削除
        let regexp = regex::Regex::new(r"---([^---]*)---").unwrap();
        let meta_removed_data = regexp.replace(&data, "");
        let parser = Parser::new(&meta_removed_data);
        let mut html_buf = String::new();
        push_html(&mut html_buf, parser);
        html! {
          <article class="markdown prose prose-slate mx-auto">
            <RawHTML inner_html={html_buf} />
          </article>
        }
      }
      FetchState::Failed(err) => html! { err },
    }
  }
}
