use yew::prelude::*;
use yew_router::prelude::*;

mod components;
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
      <div>
        <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
        <p>{ self.value }</p>
        <components::button::Button />
        <Link<routes::app_routes::AppRoutes> to={routes::app_routes::AppRoutes::Home}>{"404"}</Link<routes::app_routes::AppRoutes>>
      </div>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}
