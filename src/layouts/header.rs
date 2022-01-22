use crate::routes::app_routes::AppRoutes;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Header {
  isMenuOpen: bool,
}

pub enum Msg {
  ToggleMenu,
}

impl Component for Header {
  type Message = Msg;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Header { isMenuOpen: false }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::ToggleMenu => {
        self.isMenuOpen = !self.isMenuOpen;
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <header class="relative flex items-center justify-between py-4 lg:px-72 px-8 border-b">
        <div class="flex items-center">
          <Link<AppRoutes> classes="text-2xl font-bold" to={AppRoutes::Home}>{"mayoblog"}</Link<AppRoutes>>
        </div>
        <button class="md:hidden block" onclick={ctx.link().callback(|_| Msg::ToggleMenu)}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" width="20" height="20" viewBox="0 0 24 24"><path d="M4 22h-4v-4h4v4zm0-12h-4v4h4v-4zm0-8h-4v4h4v-4zm3 0v4h17v-4h-17zm0 12h17v-4h-17v4zm0 8h17v-4h-17v4z"/></svg>
        </button>
        {if self.isMenuOpen {
          html! {
            <nav class="absolute top-14 left-0 right-0 bg-white md:hidden flex flex-col md:flex-row items-center justify-between">
              <ul class="text-center">
                <li class="mb-2">
                  <Link<AppRoutes> classes="text-lg font-bold" to={AppRoutes::Profile}>{"Profile"}</Link<AppRoutes>>
                </li>
                <li class="mb-2">
                  <Link<AppRoutes> classes="text-lg font-bold" to={AppRoutes::Resume}>{"Resume"}</Link<AppRoutes>>
                </li>
                <li class="mb-2">
                  <a href="https://zenn.dev/mayo_dev" target="_blank" rel="noopener noreferrer" class="text-lg font-bold">{"TechBlog"}</a>
                </li>
              </ul>
            </nav>
          }
        } else {
          html! {}
        }}
        <nav class="hidden md:block">
          <ul class="flex items-center">
            <li class="ml-2">
              <Link<AppRoutes> classes="block text-blue-500 underline transition-all p-1 hover:no-underline" to={AppRoutes::Home}>{"Home"}</Link<AppRoutes>>
            </li>
            <li class="ml-2">
              <Link<AppRoutes> classes="block text-blue-500 underline transition-all p-1 hover:no-underline" to={AppRoutes::Profile}>{"Profile"}</Link<AppRoutes>>
            </li>
            <li class="ml-2">
              <Link<AppRoutes> classes="block text-blue-500 underline transition-all p-1 hover:no-underline" to={AppRoutes::Resume}>{"Resume"}</Link<AppRoutes>>
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
