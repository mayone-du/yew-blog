use crate::client::fetch::fetch_row_text;
use crate::client::state::{FetchMessage, FetchState};
use crate::components::markdown::Markdown;
use crate::components::meta_info::MetaInfo;
use crate::constants::vars::PROFILE_URL;
use crate::layouts::main_layout::MainLayout;
use crate::layouts::sidebar::Sidebar;
use crate::utils::index::{capture_val_by_regexp, create_meta_regexp};
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
      <div class="h-screen"></div>
    };
    let content = match &self.state {
      FetchState::NotFetching => loading,
      FetchState::Fetching => loading,
      FetchState::Success(data) => {
        // メタデータを抽出

        let meta_section_regexp = regex::Regex::new(r"---([^---]*)---").unwrap();

        let (meta_title_regexp, meta_description_regexp, meta_emoji_regexp, meta_updated_at_regexp) = (
          create_meta_regexp("title"),
          create_meta_regexp("description"),
          create_meta_regexp("emoji"),
          create_meta_regexp("updated_at"),
        );
        let meta_section = capture_val_by_regexp(&meta_section_regexp, &data);

        // メタデータのそれぞれの値を抽出
        let title = capture_val_by_regexp(&meta_title_regexp, &meta_section);
        let description = capture_val_by_regexp(&meta_description_regexp, &meta_section);
        let emoji = capture_val_by_regexp(&meta_emoji_regexp, &meta_section);
        let updated_at = capture_val_by_regexp(&meta_updated_at_regexp, &meta_section);

        let meta_removed_data = meta_section_regexp.replace(&data, "");

        html! {
          <div>
            <MetaInfo title={title} description={description} emoji={emoji} created_at={""} updated_at={updated_at} />
            <div class="grid grid-cols-3 lg:gap-6 gap-4">
              <div class="lg:col-span-2 col-span-3 border border-gray-200 rounded p-4 bg-white">
                <Markdown markdwon_data={meta_removed_data.to_string()} />
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
