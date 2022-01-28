use crate::client::{
  fetch::fetch_row_text,
  state::{FetchMessage, FetchState},
};
use crate::components::article_list_loading::ArticleListLoading;
use crate::constants::vars::ARTICLE_LIST_META_URL;
use crate::meta::data_list::ArticleMetaDataList;
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
    match &self.response {
      FetchState::NotFetching => html! { <ArticleListLoading loading_cols={9} /> },
      FetchState::Fetching => html! { <ArticleListLoading loading_cols={9} /> },
      FetchState::Success(data) => {
        let json_data: ArticleMetaDataList = serde_json::from_str(&data).unwrap();
        html! {
          <ul class="grid lg:gap-6 gap-4 lg:grid-cols-3 md:grid-cols-2 grid-cols-1">
            {
              json_data
              .map(|meta| {
                if meta.is_published {
                  html! {
                    <li class="col-span-1 border border-gray-200 rounded-lg shadow-sm overflow-hidden bg-white transition-all hover:bg-gray-50 hover:-translate-y-1" title={meta.title.clone()}>
                      <Link<AppRoutes> classes="block" to={AppRoutes::Article { id: meta.created_at.clone() }}>
                        <div class="lg:text-7xl text-5xl py-6 text-center bg-blue-100 border-b border-gray-100">{meta.emoji}</div>
                        <div class="p-4">
                          <h5 class="text-lg font-bold pb-3">{meta.title}</h5>
                          <p class="text-sm text-gray-500 pb-3">{meta.description}</p>
                          <small class="block text-sm text-right">{&meta.created_at}</small>
                        </div>
                      </Link<AppRoutes>>
                    </li>
                  }
                } else {
                  html! {}
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
