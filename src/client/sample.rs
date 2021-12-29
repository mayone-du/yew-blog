use graphql_client::{GraphQLQuery, Response};
use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response as Res};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

type ObjectId = String;

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
  err: JsValue,
}
impl Display for FetchError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Debug::fmt(&self.err, f)
  }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
  fn from(value: JsValue) -> Self {
    Self { err: value }
  }
}

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "./graphql/schema.graphql",
  query_path = "./graphql/all_users.graphql",
  response_derives = "Debug"
)]
struct AllUsers;

async fn fetch_users() -> Result<String, FetchError> {
  let token = "Bearer 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";
  let build_query = AllUsers::build_query(all_users::Variables {
    token: token.to_string(),
  });
  let query = serde_json::json!(build_query);
  let mut opts = RequestInit::new();
  opts.method("POST");
  opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
  opts.mode(RequestMode::Cors);
  let url = String::from("https://api.github.com/graphql");
  let request = Request::new_with_str_and_init(url.as_str(), &opts)?;
  request.headers().set("Authorization", token)?;

  let window = yew::utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp: Res = resp_value.dyn_into().unwrap();

  let text = JsFuture::from(resp.text()?).await?;
  Ok(text.as_string().unwrap())
}

pub struct Users {
  list: String,
  link: ComponentLink<Self>,
}

pub enum Msg {
  UpdateList(String),
}

impl Component for Users {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      list: String::new(),
      link,
    }
  }

  fn rendered(&mut self, first_render: bool) {
    let link = self.link.clone();
    if first_render {
      spawn_local(async move {
        let res = fetch_users().await;
        link.send_message(Msg::UpdateList(res.unwrap()))
      });
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::UpdateList(res) => {
        self.list = res;
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Welcome..." }</h1>
                    <h2 class="subtitle">{ "...to the best yew content" }</h2>
                    <h3> { self.list.clone() } </h3>
                </div>
            </div>
        </div>
    }
  }
}
