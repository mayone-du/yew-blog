use crate::client::fetch::fetch_row_text;
use crate::client::state::{FetchMessage, FetchState};
use crate::components::markdown::Markdown;
use crate::constants::vars::RESUME_URL;
use crate::layouts::main_layout::MainLayout;
use crate::layouts::sidebar::Sidebar;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ResumePage {
  state: FetchState<String>,
}

impl Component for ResumePage {
  type Message = FetchMessage;
  type Properties = Props;

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_message(FetchMessage::FetchStart);
    Self {
      state: FetchState::NotFetching,
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      FetchMessage::SetFetchState(fetch_state) => {
        self.state = fetch_state;
        true
      }
      FetchMessage::FetchStart => {
        ctx.link().send_future(async move {
          match fetch_row_text(RESUME_URL).await {
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
    let loading = html! { <div>{"Loading..."}</div> };
    let content = match &self.state {
      FetchState::NotFetching => loading,
      FetchState::Fetching => loading,
      FetchState::Success(data) => {
        html! {
          <div>
            <div class="grid grid-cols-3 lg:gap-6 gap-4">
              <div class="lg:col-span-2 col-span-3 border border-gray-100 rounded-lg p-4 bg-white">
                <Markdown markdwon_data={data.to_string()} />
              </div>
              <Sidebar />
            </div>
          </div>
        }
      }
      FetchState::Failed(err) => html! { err },
    };
    html! {
      <MainLayout>
        {content}
      </MainLayout>
    }
  }
}
