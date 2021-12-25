use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod layouts;
mod pages;
mod routes;

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
        // the value has changed so we need to
        // re-render for it to appear on the page
        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
    let link = ctx.link();
    html! {
      <BrowserRouter>
      <Switch<routes::app_routes::AppRoutes> render={Switch::render(switch)} />

        <div>
          <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
          <p>{ self.value }</p>
          <components::button::Button />
          <Link<routes::app_routes::AppRoutes> to={routes::app_routes::AppRoutes::Home}>{"go home"}</Link<routes::app_routes::AppRoutes>>
        </div>
      </BrowserRouter>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}

// TODO: 同じパスへ遷移したときは再レンダリングとかhistory pushはいらない？
fn switch(routes: &routes::app_routes::AppRoutes) -> Html {
  match routes {
    routes::app_routes::AppRoutes::Home => html! { <pages::index::IndexPage /> },
    routes::app_routes::AppRoutes::Profile => html! { <pages::profile::ProfilePage /> },
    routes::app_routes::AppRoutes::NotFound => html! { <h1>{ "404" }</h1> },
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
