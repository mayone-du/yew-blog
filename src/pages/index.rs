use crate::components::article_list::ArticleList;
use crate::layouts::footer::Footer;
use crate::layouts::header::Header;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct IndexPage;

impl Component for IndexPage {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <Header />
        <main class="px-80 py-6">
          <h1 class="text-4xl font-bold py-6 text-center">{"ひまたんしかかたん🥺"}</h1>
          <ArticleList />
        </main>
        <Footer />
      </div>
    }
  }
}
