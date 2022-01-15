use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ProfileCard;

impl Component for ProfileCard {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="shadow-sm border border-gray-200 rounded p-4 mb-6">
        <div class="flex items-center">
          <img class="inline-block object-cover w-14 h-14 rounded-full mr-2" src="https://raw.githubusercontent.com/mayone-du/blog-contents/main/images/profile.jpg" alt="プロフィール" />
          <div>
            <h4 class="font-bold text-lg">{"まよねーづ"}</h4>
            <a href="https://twitter.com/mayo__dev" class="inline-block text-gray-300" target="_blank" rel="noopener noreferrer">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24"><path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"/></svg>
            </a>
          </div>
        </div>
        <p class="pt-2 text-sm text-gray-700">{"高校卒業後、独学でWebエンジニアになった19歳。ReactやTypeScriptを用いたフロントエンド開発が得意。"}</p>
      </div>
    }
  }
}
