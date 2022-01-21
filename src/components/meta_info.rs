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
    let created_at_length = &ctx.props().created_at.chars().count();
    let updated_at_length = &ctx.props().updated_at.chars().count();
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
        {
          if created_at_length > &0 {
            html! {
              <div class="flex items-center justify-center text-gray-600 pb-8">
                <small class="flex items-center text-sm">
                  <svg xmlns="http://www.w3.org/2000/svg" class="mr-1 block" width="16" height="16" viewBox="0 0 24 24"><path fill="currentColor" d="M18.2.7c-.8 0-1.4.5-1.4 1.3v3.1c0 .7.7 1.3 1.4 1.3s1.4-.5 1.4-1.3V2c0-.7-.7-1.3-1.4-1.3zM5.8.7C5 .7 4.4 1.3 4.4 2v3.1c0 .7.7 1.3 1.4 1.3s1.4-.5 1.4-1.3V2C7.3 1.3 6.6.7 5.8.7z"></path><path fill="currentColor" d="M21.7 3.6H2.3c-1.1 0-2 .9-2 2v15.7c0 1.1.9 2 2 2h19.4c1.1 0 2-.9 2-2V5.5c-.1-1-.9-1.9-2-1.9zm-.3 16.2c0 .6-.5 1.1-1.1 1.1H3.4c-.5 0-.9-.4-.9-.9V9.7h19v10.1z"></path></svg>
                  {&ctx.props().created_at} {" に公開"}
                </small>
              </div>
            }
          } else {
            html! {}
          }
        }
        {
          if updated_at_length > &0 {
            html! {
            <div class="flex items-center justify-center text-gray-600 pb-8">
              <small class="flex items-center text-sm">
                <svg xmlns="http://www.w3.org/2000/svg" class="mr-1 block" width="16" height="16" viewBox="0 0 24 24"><g fill="currentColor"><path d="M2.9 10H8c.2 0 .4-.1.6-.3.3-.3.3-.7 0-1.1l-2.2-2C7.9 5.1 9.9 4.3 12 4.3c4 0 7.2 2.9 7.7 6.8.1.7.7 1.1 1.4 1.1h.1c.8-.1 1.2-.8 1.1-1.5-.5-5.2-5-9.1-10.3-9.1-2.8 0-5.5 1.1-7.4 3.1L2.8 2.9c-.3-.3-.7-.3-1.1 0-.2.2-.2.4-.2.6v5.1c0 .7.6 1.4 1.4 1.4zM21.1 14H16c-.2 0-.4.1-.6.3-.3.3-.3.7 0 1.1l2.1 2.1C16 19 14 19.8 11.9 19.8c-4 0-7.2-2.9-7.7-6.9-.2-.7-.9-1.3-1.6-1.2-.8.1-1.2.8-1.1 1.5.5 5.2 5 9.1 10.3 9.1 2.8 0 5.5-1.1 7.4-3.1L21 21c.3.3.7.3 1.1 0 .1-.1.3-.3.3-.6v-5.1c.1-.6-.5-1.3-1.3-1.3z"></path></g></svg>
                {&ctx.props().updated_at} {" に更新"}
              </small>
            </div>
            }
          } else {
            html! {}
          }
        }
      </div>
    }
  }
}
