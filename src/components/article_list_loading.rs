use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  pub loading_cols: u8,
}

pub struct ArticleListLoading;

impl Component for ArticleListLoading {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let loading_column = html! {
      <li class="col-span-1 h-60 rounded-lg shadow-sm overflow-hidden bg-white">
        <div class="bg-blue-100 border-b border-gray-100 w-full py-6">
          <div class="w-[72px] h-[72px] mx-auto rounded-full bg-gray-300 animate-pulse"></div>
        </div>
        <div class="p-4">
          <div class="animate-pulse bg-gray-300 w-full h-6 mb-3 rounded"></div>
          <div class="animate-pulse bg-gray-300 w-full h-4 mb-1 rounded"></div>
          <div class="animate-pulse bg-gray-300 w-full h-4 mb-3 rounded"></div>
          <div class="animate-pulse bg-gray-300 w-20 h-3 ml-auto rounded"></div>
        </div>
      </li>
    };
    html! {
      <ul class="grid lg:gap-6 gap-4 lg:grid-cols-3 md:grid-cols-2 grid-cols-1">
        {for (0..ctx.props().loading_cols).map(|_| loading_column.clone())}
      </ul>
    }
  }
}
