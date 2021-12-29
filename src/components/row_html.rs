use web_sys::Node;
use yew::{virtual_dom::VNode, Component, Context, Html, Properties};

pub enum Msg {}

// TODO: Props Name
#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHTMLProps {
  pub inner_html: String,
}

pub struct RawHTML {
  props: RawHTMLProps,
}

impl Component for RawHTML {
  type Message = Msg;
  type Properties = RawHTMLProps;

  fn create(ctx: &Context<Self>) -> Self {
    RawHTML {
      props: RawHTMLProps {
        inner_html: ctx.props().inner_html.clone(),
      },
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
    true
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    let div = web_sys::window()
      .unwrap()
      .document()
      .unwrap()
      .create_element("div")
      .unwrap();
    div.set_inner_html(&self.props.inner_html[..]);

    let node = Node::from(div);
    let vnode = VNode::VRef(node);
    vnode
  }
}
