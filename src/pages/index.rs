use crate::components::markdown::Markdown;
use crate::layouts::main_layout::MainLayout;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct IndexPage;

impl Component for IndexPage {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    IndexPage
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <MainLayout>
        <h1>{"Index Page"}</h1>
        <Markdown />
      </MainLayout>
    }
  }
}
