use crate::routes::app_routes::AppRoutes;
use yew::{html, Component, Context, Html, Properties};
use yew_router::components::Link;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Header;

enum Msg {}

impl Component for Header {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Header
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <header>
        <nav>
          <ul>
            <li>
              <Link<AppRoutes> to={AppRoutes::Profile}>{"Profile"}</Link<AppRoutes>>
            </li>
          </ul>
        </nav>
      </header>
    }
  }
}
