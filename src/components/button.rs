use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Button;

enum Msg {
  Reset,
}

impl Component for Button {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Button
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <button>{"hoge"}</button>
    }
  }
}
