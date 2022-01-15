use crate::client::{fetch::fetch_row_text, state::FetchMessage, state::FetchState};
use crate::components::markdown::Markdown;
use crate::layouts::main_layout::MainLayout;
use crate::layouts::sidebar::Sidebar;
use crate::meta::data_list::ArticleMetaData;
use regex;
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
        let id = ctx.props().id.clone();
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

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let loading = html! {
      <div class="animate-pulse bg-gray-300 w-full h-8"></div>
    };
    match &self.state {
      FetchState::NotFetching => loading,
      FetchState::Fetching => loading,
      FetchState::Success(data) => {
        // メタデータを抽出
        let regexp = regex::Regex::new(r"---([^---]*)---").unwrap();
        let meta_data_str = regexp.captures(&data).unwrap().get(1).unwrap().as_str();
        // TODO: メタデータのみを抽出
        let meta_data = ArticleMetaData {
          title: meta_data_str.replace("title: ", ""),
          description: meta_data_str.replace("description: ", ""),
          emoji: meta_data_str.replace("emoji: ", ""),
          created_at: "created_at: ".to_string(),
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
