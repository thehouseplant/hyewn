use yew::prelude::*;

pub struct New;

impl Component for New {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "New" }</h1>
        }
    }
}