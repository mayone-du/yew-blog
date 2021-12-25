use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ProfilePage;

impl Component for ProfilePage {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    ProfilePage
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div>{"ProfilePage"}</div>
    }
  }
}
