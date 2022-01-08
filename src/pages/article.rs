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
    ArticlePage
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let id = ctx.props().id.clone();
    html! {
      <MainLayout>
        <h1>{"Article Page"}</h1>
        {id}
      </MainLayout>
    }
  }
}
