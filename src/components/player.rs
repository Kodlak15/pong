use yew::{html, Children, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub children: Children,
}

pub struct Player;

impl Component for Player {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={ctx.props().id.clone()}>
                { ctx.props().children.clone() }
            </div>
        }
    }
}
