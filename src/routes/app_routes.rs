use yew_router::prelude::*;

// Note: GitHub Pagesでホスティングするため、仕方なく/yew-blogをつけている。本当はなしにしたい。
#[derive(Clone, Routable, PartialEq)]
pub enum AppRoutes {
  #[at("/yew-blog")]
  Home,
  #[at("/yew-blog/profile")]
  Profile,
  #[at("/yew-blog/resume")]
  Resume,
  #[at("/yew-blog/article/:id")]
  Article { id: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}
