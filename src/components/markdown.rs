// use pulldown_cmark::{html::push_html, Parser};
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Markdown;

// enum Msg {}

impl Component for Markdown {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Markdown
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <article>
        {"blog"}
      </article>
    }
  }
}
