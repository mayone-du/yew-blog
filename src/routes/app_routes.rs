use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoutes {
  #[at("/")]
  Home,
  #[not_found]
  #[at("/404")]
  NotFound,
}
