use crate::components::row_html::RawHTML;
use pulldown_cmark::{html::push_html, Parser};
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Markdown;

pub enum Msg {}

impl Component for Markdown {
  type Message = Msg;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Markdown
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let content = include_str!("../../data/articles/first.md");
    let parser = Parser::new(&content);
    let mut html_buf = String::new();
    push_html(&mut html_buf, parser);
    html! {
      <article>
        <RawHTML inner_html={html_buf} />
      </article>
    }
  }
}
