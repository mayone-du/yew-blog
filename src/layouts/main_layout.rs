use crate::layouts::header::Header;
use crate::layouts::sidebar::Sidebar;
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

        <div class="grid grid-cols-3 gap-4">
          <main class="col-span-2 border border-blue-300">
            { for ctx.props().children.iter() }
          </main>
          <Sidebar />
        </div>
      </div>
    }
  }
}
