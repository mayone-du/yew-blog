use crate::routes::app_routes::AppRoutes;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Header;

impl Component for Header {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Header
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <header class="flex items-center justify-between px-80 border-b">
        <div class="flex items-center">
          <h1 class="text-2xl font-bold">{"mayoblog"}</h1>
        </div>
        <nav>
          <ul class="flex items-center">
            <li>
              <Link<AppRoutes> classes="ml-2 block text-blue-500 underline" to={AppRoutes::Home}>{"Home"}</Link<AppRoutes>>
            </li>
            <li>
              <Link<AppRoutes> classes="ml-2 block text-blue-500 underline" to={AppRoutes::Profile}>{"Profile"}</Link<AppRoutes>>
            </li>
          </ul>
        </nav>
      </header>
    }
  }
}
