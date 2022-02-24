use yew::prelude::*;
use yew_router::prelude::*;

mod client;
mod components;
mod constants;
mod layouts;
mod meta;
mod pages;
mod routes;
mod utils;

use routes::app_routes::AppRoutes;

struct Model;

impl Component for Model {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    false
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <BrowserRouter>
        <Switch<AppRoutes> render={Switch::render(switch)} />
      </BrowserRouter>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}

fn switch(routes: &AppRoutes) -> Html {
  match routes {
    AppRoutes::Home => html! { <pages::index::IndexPage /> },
    AppRoutes::Profile => html! { <pages::profile::ProfilePage /> },
    AppRoutes::Resume => html! { <pages::resume::ResumePage /> },
    AppRoutes::Article { id } => html! { <pages::article::ArticlePage id={id.clone()} /> },
    AppRoutes::NotFound => html! { <h1>{ "404 Not Found :( By Yew" }</h1> },
  }
}

// #[function_component(Main)]
// fn app() -> Html {
//   html! {
//     <BrowserRouter>
//       <Switch<routes::app_routes::AppRoutes> render={Switch::render(switch)} />
//     </BrowserRouter>
//   }
// }
