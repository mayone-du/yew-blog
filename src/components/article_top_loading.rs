use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ArticleTopLoading;

impl Component for ArticleTopLoading {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <div class="w-full h-screen">
          <div class="animate-pulse bg-gray-300 rounded-full w-28 h-28 mb-4 mx-auto"></div>
          <div class="animate-pulse bg-gray-300 w-2/3 h-8 mb-4 mx-auto"></div>
          <div class="animate-pulse bg-gray-300 w-1/2 h-4 mb-6 mx-auto"></div>
          <div class="animate-pulse bg-gray-300 w-24 h-3 mx-auto"></div>
        </div>
    }
  }
}
