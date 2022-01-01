use crate::routes::app_routes::AppRoutes;
use gloo::utils;
use yew::{function_component, html, use_effect, use_state, Callback};
use yew_router::hooks::use_route;

#[function_component(UseEffect)]
fn effect() -> Html {
  let counter = use_state(|| 0);
  let current_route = use_route::<AppRoutes>();

  {
    let counter = counter.clone();
    use_effect(move || {
      // Make a call to DOM API after component is rendered
      utils::document().set_title(&format!("You clicked {} times", *counter));

      // Perform the cleanup
      || utils::document().set_title("You clicked 0 times")
    });
  }
  let onclick = {
    let counter = counter.clone();
    Callback::from(move |_| counter.set(*counter + 1))
  };

  html! {
      <button {onclick}>{ format!("Increment to {}", *counter) }</button>
  }
}

#[function_component(Provider)]
pub fn provider() -> Html {
  html! {
    <div>
      {"hoge"}
    </div>
  }
}
