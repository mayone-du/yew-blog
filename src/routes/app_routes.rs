use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoutes {
  #[at("/")]
  Home,
  #[at("/profile")]
  Profile,
  #[not_found]
  #[at("/404")]
  NotFound,
}
