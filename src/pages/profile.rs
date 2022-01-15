use crate::client::fetch::fetch_row_text;
use crate::client::state::{FetchMessage, FetchState};
use crate::components::markdown::Markdown;
use crate::constants::vars::PROFILE_URL;
use crate::layouts::main_layout::MainLayout;
use crate::layouts::sidebar::Sidebar;
use crate::meta::data_list::OtherMetaData;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ProfilePage {
  state: FetchState<String>,
}

impl Component for ProfilePage {
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
          match fetch_row_text(PROFILE_URL).await {
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
      <div>{"Loading..."}</div>
    };
    match &self.state {
      FetchState::NotFetching => loading,
      FetchState::Fetching => loading,
      FetchState::Success(data) => {
        // メタデータを抽出
        let regexp = regex::Regex::new(r"---([^---]*)---").unwrap();
        let meta_data_str = regexp.captures(&data).unwrap().get(1).unwrap().as_str();
        // TODO: メタデータのみを抽出
        let meta_data = OtherMetaData {
          title: meta_data_str.replace("title: ", ""),
          description: meta_data_str.replace("description: ", ""),
          emoji: meta_data_str.replace("emoji: ", ""),
          is_published: true,
        };
        let meta_removed_data = regexp.replace(&data, "");
        html! {
          <MainLayout>
            {meta_data.emoji}
            <div class="grid grid-cols-3 gap-6">
              <div class="col-span-2 border border-gray-200 rounded p-4 bg-white">
                <Markdown markdwon_data={meta_removed_data.to_string()} />
              </div>
              <Sidebar />
            </div>
          </MainLayout>
        }
      }
      FetchState::Failed(err) => html! { err },
    }
  }
}
