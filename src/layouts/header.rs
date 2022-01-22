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
      <header class="flex items-center justify-between py-4 lg:px-72 px-8 border-b">
        <div class="flex items-center">
          <Link<AppRoutes> classes="text-2xl font-bold" to={AppRoutes::Home}>{"mayoblog"}</Link<AppRoutes>>
        </div>
        <nav>
          <ul class="flex items-center">
            <li class="ml-2">
              <Link<AppRoutes> classes="block text-blue-500 underline transition-all p-1 hover:no-underline" to={AppRoutes::Home}>{"Home"}</Link<AppRoutes>>
            </li>
            <li class="ml-2">
              <Link<AppRoutes> classes="block text-blue-500 underline transition-all p-1 hover:no-underline" to={AppRoutes::Profile}>{"Profile"}</Link<AppRoutes>>
            </li>
            <li class="ml-2">
              <a href="https://zenn.dev/mayo_dev" target="_blank" rel="noopener noreferrer" class="block text-blue-500 underline transition-all p-1 hover:no-underline">{"TechBlog"}</a>
            </li>
          </ul>
        </nav>
      </header>
    }
  }
}
