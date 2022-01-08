use crate::client::fetch_raw_text::fetch_row_text;
use crate::client::state::FetchState;
use crate::constants::vars::ARTICLE_LIST_META_URL;
use crate::meta::data_list::MetaDataList;
use crate::routes::app_routes::AppRoutes;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

#[derive(PartialEq, Properties)]
pub struct Props;

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

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_message(Msg::GetMarkdown);
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

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let loading = html! {
      <div class="animate-pulse bg-gray-300 w-full h-8"></div>
    };
    match &self.markdown {
      FetchState::NotFetching => loading,
      FetchState::Fetching => loading,
      FetchState::Success(data) => {
        let json_data: MetaDataList = serde_json::from_str(&data).unwrap();
        html! {
          <ul>
            {
              json_data
              .map(|meta| {
                html! {
                  <li>
                    <Link<AppRoutes> to={AppRoutes::Article { id: meta.created_at.clone() }}>
                      { &meta.created_at }
                    </Link<AppRoutes>>
                    <h1 class="font-bold text-3xl">{meta.title}</h1>
                    <p>{meta.description}</p>
                  </li>
                }
              })
              .collect::<Html>()
            }
          </ul>
        }
      }
      FetchState::Failed(err) => html! { err },
    }
  }
}
