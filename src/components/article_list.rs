use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ArticleList;

pub enum Msg {}

impl Component for ArticleList {
  type Message = Msg;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    ArticleList
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div>
        {"article list!"}
      </div>
    }
  }
}
