use graphql_client::reqwest::post_graphql_blocking;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, RequestBuilder};
use std::error::Error;
// use std::thread;
use wasm_bindgen::JsValue;
use yew::{html, Component, Context, Html, Properties};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "src/client/graphql/schema.docs.graphql",
  query_path = "src/client/graphql/query/view_repo.graphql",
  response_derives = "Debug"
)]
pub struct ViewRepo;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct ArticleList;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

// async fn perform_my_query(variables: view_repo::Variables) -> Result<String, Box<dyn Error>> {
//   // this is the important line
//   let request_body = ViewRepo::build_query(variables);

//   let client = Client::new();
//   let res = client
//     .post("https://api.github.com/")
//     .json(&request_body)
//     .send()
//     .await?;
//   println!("{:?}", res);
//   // println!("{:?}", res.text().await?);
//   // let response_body: Response<view_repo::ResponseData> = res.json().await?;
//   // println!("{:#?}", response_body);
//   Ok(res.text().await?)
// }

pub enum Msg {}

impl Component for ArticleList {
  type Message = Msg;
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    ArticleList
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let variables = view_repo::Variables {
      owner: "rust-lang".to_string(),
      name: "rust".to_string(),
    };
    // let a = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1");
    // let aaa = match a {
    //   Ok(a) => a.text().unwrap(),
    //   Err(e) => "error".to_string(),
    // };
    // web_sys::console::log_1(&JsValue::from_str(aaa.as_str()));

    let github_api_token =
      std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    // let client = Client::builder()
    //   .user_agent("graphql-rust/0.10.0")
    //   .default_headers(
    //     std::iter::once((
    //       reqwest::header::AUTHORIZATION,
    //       reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token)).unwrap(),
    //     ))
    //     .collect(),
    //   )
    //   .build()
    //   .unwrap();

    let client = reqwest::blocking::Client::new();

    let response_body =
      post_graphql_blocking::<ViewRepo, _>(&client, "https://api.github.com/graphql", variables)
        .unwrap();

    // let result = perform_my_query(variables);
    // web_sys::console::log_1(&JsValue::from_str("hogeeee"));
    // let handle = thread::spawn(|| async {
    //   let result = perform_my_query(variables).await;
    // });
    // handle.join().expect("join error");
    html! {
      <div>
      // {result}
        {"article list!"}
      </div>
    }
  }
}
