use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoutes {
  #[at("/yew-blog")]
  Home,
  #[at("/yew-blog/profile")]
  Profile,
  #[not_found]
  #[at("/404")]
  NotFound,
}
