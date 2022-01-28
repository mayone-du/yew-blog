use crate::client::{fetch::fetch_row_text, state::FetchMessage, state::FetchState};
use crate::components::markdown::Markdown;
use crate::components::meta_info::MetaInfo;
use crate::components::meta_info_loading::MetaInfoLoading;
use crate::layouts::main_layout::MainLayout;
use crate::layouts::sidebar::Sidebar;
use crate::utils::index::{capture_val_by_regexp, create_meta_regexp, create_meta_section_regexp};
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub id: String,
}

pub struct ArticlePage {
  state: FetchState<String>,
}

impl Component for ArticlePage {
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
        // TODO: いい感じに文字列連結したかった Rustの最新ver(1.58とか？)なら文字列連結できるようになってるかも。
        let id = &ctx.props().id;
        let year = id.split("-").next().unwrap(); // or let year = id.split("-").next().unwrap();
        let request_url = format!(
          "https://raw.githubusercontent.com/mayone-du/blog-contents/main/articles/{}/{}.md",
          year,
          ctx.props().id
        );
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

  fn view(&self, ctx: &Context<Self>) -> Html {
    let content = match &self.state {
      FetchState::NotFetching => html! { <MetaInfoLoading /> },
      FetchState::Fetching => html! { <MetaInfoLoading /> },
      FetchState::Success(data) => {
        if &data == &"404: Not Found" {
          html! { <div>{data}</div> }
        } else {
          // メタデータを抽出
          let meta_section_regexp = create_meta_section_regexp();

          let (
            meta_title_regexp,
            meta_description_regexp,
            meta_emoji_regexp,
            meta_is_published_regexp,
          ) = (
            create_meta_regexp("title"),
            create_meta_regexp("description"),
            create_meta_regexp("emoji"),
            create_meta_regexp("is_published"),
          );
          let meta_section = capture_val_by_regexp(&meta_section_regexp, &data);

          // メタデータのそれぞれの値を抽出
          let title = capture_val_by_regexp(&meta_title_regexp, &meta_section);
          let description = capture_val_by_regexp(&meta_description_regexp, &meta_section);
          let emoji = capture_val_by_regexp(&meta_emoji_regexp, &meta_section);
          let is_published = capture_val_by_regexp(&meta_is_published_regexp, &meta_section);

          if is_published == "false" {
            html! { <div>{"This page is not published."}</div> }
          } else {
            let meta_removed_data = meta_section_regexp.replace(&data, "");
            html! {
              <div>
                <MetaInfo title={title} description={description} emoji={emoji} created_at={ctx.props().id.clone()} updated_at={""} />
                <div class="grid grid-cols-3 lg:gap-6 gap-4">
                  <div class="lg:col-span-2 col-span-3 border border-gray-100 rounded-lg p-4 bg-white">
                    <Markdown markdwon_data={meta_removed_data.to_string()} />
                  </div>
                  <Sidebar />
                </div>
              </div>
            }
          }
        }
      }
      FetchState::Failed(err) => html! { <div class="font-bold text-red-600">{err}</div> },
    };
    html! {
      <MainLayout>{content}</MainLayout>
    }
  }
}
