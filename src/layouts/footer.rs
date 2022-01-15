use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Footer;

impl Component for Footer {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Footer
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <footer class="flex items-center justify-center lg:px-80 px-10 py-4 bg-gray-800 text-white">
        <p>{"copyright Yuki Ishikawa"}</p>
      </footer>
    }
  }
}
