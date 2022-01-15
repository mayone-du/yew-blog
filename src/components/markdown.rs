use crate::components::row_html::RawHTML;
use pulldown_cmark::{html::push_html, Parser};
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub markdwon_data: String,
}

pub struct Markdown;

impl Component for Markdown {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let parser = Parser::new(&ctx.props().markdwon_data);
    let mut html_buf = String::new();
    push_html(&mut html_buf, parser);
    html! {
      <article class="prose prose-slate mx-auto">
        <RawHTML inner_html={html_buf} />
      </article>
    }
  }
}
