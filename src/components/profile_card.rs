use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ProfileCard;

impl Component for ProfileCard {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="shadow-sm rounded p-4">
        <h4>{"Profile"}</h4>
        <p>{"まよねーづ"}</p>
      </div>
    }
  }
}
