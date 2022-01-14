use crate::components::markdown::Markdown;
use crate::constants::vars::PROFILE_URL;
use crate::layouts::main_layout::MainLayout;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ProfilePage;

impl Component for ProfilePage {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <MainLayout>
        <Markdown request_url={PROFILE_URL} />
      </MainLayout>
    }
  }
}
