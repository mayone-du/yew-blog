use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub title: String,
  pub description: String,
  pub emoji: String,
  pub created_at: String,
  pub updated_at: String,
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
        <div class="text-center font-bold text-8xl pt-2">
          {&ctx.props().emoji}
        </div>
        <h1 class="font-bold text-3xl text-center py-4">
          {&ctx.props().title}
        </h1>
        <p class="pb-4 text-sm text-center text-gray-600">
          {&ctx.props().description}
        </p>
        <div class="flex items-center justify-center pb-8">
          <small>
            {&ctx.props().created_at}
          </small>
        </div>
      </div>
    }
  }
}
