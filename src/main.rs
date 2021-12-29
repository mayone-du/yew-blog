use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod layouts;
mod pages;
mod routes;

use routes::app_routes::AppRoutes;

enum Msg {
  AddOne,
}

struct Model {
  value: i64,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self { value: 0 }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::AddOne => {
        self.value += 1;
        true
      }
    }
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
    AppRoutes::NotFound => html! { <h1>{ "404" }</h1> },
  }
}

#[function_component(Main)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<routes::app_routes::AppRoutes> render={Switch::render(switch)} />
    </BrowserRouter>
  }
}
