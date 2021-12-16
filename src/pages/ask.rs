use yew::prelude::*;

pub struct Ask;

impl Component for Ask {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Ask" }</h1>
        }
    }
}