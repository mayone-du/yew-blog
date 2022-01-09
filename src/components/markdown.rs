use crate::client::{fetch::fetch_row_text, state::FetchMessage, state::FetchState};
use crate::components::row_html::RawHTML;
use crate::constants::vars::ARTICLE_BASE_URL;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub id: String,
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
        ctx.link().send_future(async {
          // let api_url: &str = format!("{}{}.md", ARTICLE_BASE_URL, ctx.props().id);
          // let api_url = format!(
          //   "https://raw.githubusercontent.com/mayone-du/blog-contents/main/articles/{}.md",
          //   ctx.props().id
          // )
          // .as_str();
          let a: &str = "https://raw.githubusercontent.com/mayone-du/blog-contents/main/articles/2022/2022-01-08.md";
          match fetch_row_text(a).await {
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
        html! {
          <article class="markdown prose prose-slate">
            // TODO: meta情報を削除
            <RawHTML inner_html={data.to_string().clone()} />
          </article>
        }
      }
      FetchState::Failed(err) => html! { err },
    }
  }
}
