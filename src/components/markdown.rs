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
    let content = std::fs::read_to_string("src/articles/first.md");
    let content = match content {
      Ok(content) => {
        println!("{}", content);
        content
      }
      Err(e) => {
        println!("{}", e);
        "Markdown Error!".to_string()
      }
    };
    // let parser = Parser::new(&content);
    // let mut html_buf = String::new();
    // push_html(&mut html_buf, parser);
    // println!("hogeeeeeeeeeeeeeee{}", content);
    // println!("{}", html_buf);
    html! {
      <article>
      {content}
        // {html_buf}
      </article>
    }
  }
}
