use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Markdown;

enum Msg {}

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
        html! {
          <div>
            {content}
          </div>
        }
      }
      Err(e) => {
        eprintln!("{}", e);
        html! {
          <div>
            {e.to_string()}
          </div>
        }
      }
    };
    html! {
      <article>
        {content}
      </article>
    }
  }
}
