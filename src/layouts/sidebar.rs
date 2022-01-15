use crate::components::profile_card::ProfileCard;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Sidebar;

impl Component for Sidebar {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Sidebar
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <aside class="block col-span-1">
        <ProfileCard />
      </aside>
    }
  }
}
