use crate::meta::data_list::ArticleMetaData;
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub meta_data: String,
}

pub struct Markdown;

impl Component for Markdown {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div>
        {ctx.props().meta_data.clone()}
      </div>
    }
  }
}
