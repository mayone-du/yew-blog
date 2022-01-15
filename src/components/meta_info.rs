use crate::meta::data_list::ArticleMetaData;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub title: String,
  pub description: String,
  pub emoji: String,
  pub created_at: String,
  // pub updated_at: String,
}

pub struct MetaInfo;

impl Component for MetaInfo {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        <div class="text-center font-bold text-4xl">
          {&ctx.props().emoji}
        </div>
        <h1 class="font-bold text-3xl text-center pt-4 pb-8">
          {&ctx.props().title}
        </h1>
        <p class="text-sm text-gray-600">
          {&ctx.props().description}
        </p>
      </div>
    }
  }
}
