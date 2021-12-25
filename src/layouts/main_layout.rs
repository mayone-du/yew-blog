use crate::components::header::Header;
use yew::{html, Children, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
}

pub struct MainLayout;

impl Component for MainLayout {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <Header />
        <main>
          { for ctx.props().children.iter() }
        </main>
      </div>
    }
  }
}
