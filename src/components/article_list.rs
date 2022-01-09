use crate::client::{
  fetch::fetch_row_text,
  state::{FetchMessage, FetchState},
};
use crate::constants::vars::ARTICLE_LIST_META_URL;
use crate::meta::data_list::MetaDataList;
use crate::routes::app_routes::AppRoutes;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ArticleList {
  response: FetchState<String>,
}

impl Component for ArticleList {
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
          match fetch_row_text(ARTICLE_LIST_META_URL).await {
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
        let json_data: MetaDataList = serde_json::from_str(&data).unwrap();
        html! {
          <ul>
            {
              json_data
              .map(|meta| {
                html! {
                  <li>
                    <Link<AppRoutes> classes="block text-blue-500 hover:bg-gray-100" to={AppRoutes::Article { id: meta.created_at.clone() }}>
                      <span class="block font-bold" title={meta.title.clone()}>{meta.title}</span>
                      <span class="block">{meta.emoji}</span>
                      <span class="block">{meta.description}</span>
                      <span class="text-sm">{&meta.created_at}</span>
                    </Link<AppRoutes>>
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
