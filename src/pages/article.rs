use crate::components::markdown::Markdown;
use crate::layouts::main_layout::MainLayout;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub id: String,
}

pub struct ArticlePage;

impl Component for ArticlePage {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // TODO: いい感じに文字列連結したかった
    let id = ctx.props().id.clone();
    let year = id.split("-").next().unwrap(); // or let year = id.split("-").next().unwrap();
    let request_url = format!(
      "https://raw.githubusercontent.com/mayone-du/blog-contents/main/articles/{}/{}.md",
      year,
      ctx.props().id
    );
    html! {
      <MainLayout>
        <h1>{"Article Page"}</h1>
        <Markdown request_url={request_url} />
      </MainLayout>
    }
  }
}
