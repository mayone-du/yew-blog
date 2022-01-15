use crate::components::article_list::ArticleList;
use crate::layouts::main_layout::MainLayout;
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
      <MainLayout>
        <h1 class="lg:text-4xl text-2xl font-bold pt-6 pb-12 text-center">{"ひまたんしかかたん🥺"}</h1>
        <ArticleList />
      </MainLayout>
    }
  }
}
